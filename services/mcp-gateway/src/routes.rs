use axum::{http::StatusCode, Json, extract::State};
use serde_json::json;
use validator::Validate;

use crate::{
    gpt_client::{call_gpt, call_local_model},
    model::*,
};

/// Basic health endpoint
pub async fn health() -> &'static str {
    "ok"
}

/// Input sanitization & preprocessing step
pub async fn query(Json(mut req): Json<QueryRequest>) -> Result<Json<ReasoningRequest>, (StatusCode, Json<serde_json::Value>)> {
    // Validate input
    if let Err(validation_errors) = req.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "Validation failed",
                "details": validation_errors
            }))
        ));
    }

    // Sanitize and validate
    if let Err(e) = model::validation::validate_query_request(&mut req) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "Input validation failed",
                "details": e.to_string()
            }))
        ));
    }

    let reason_req = ReasoningRequest {
        prompt: format!("Summarize and reason about: {}", req.text),
        context: req.files.unwrap_or_default(),
    };
    
    Ok(Json(reason_req))
}

/// Online reasoning via GPT-5 (preferred if API key available)
pub async fn reason(Json(mut reason_req): Json<ReasoningRequest>) -> Result<(StatusCode, Json<ReasoningResponse>), (StatusCode, Json<serde_json::Value>)> {
    // Validate input
    if let Err(validation_errors) = reason_req.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "Validation failed",
                "details": validation_errors
            }))
        ));
    }

    // Sanitize and validate
    if let Err(e) = model::validation::validate_reasoning_request(&mut reason_req) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "Input validation failed",
                "details": e.to_string()
            }))
        ));
    }

    match call_gpt(&reason_req).await {
        Ok(res) => Ok((StatusCode::OK, Json(res))),
        Err(err) => Ok((
            StatusCode::BAD_GATEWAY,
            Json(ReasoningResponse {
                summary: format!("Online reasoning error: {err}"),
                ..ReasoningResponse::default()
            }),
        )),
    }
}

/// Offline reasoning via local llama.cpp model (fallback)
pub async fn reason_local(Json(mut req): Json<ReasoningRequest>) -> Result<(StatusCode, Json<ReasoningResponse>), (StatusCode, Json<serde_json::Value>)> {
    // Validate input
    if let Err(validation_errors) = req.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "Validation failed",
                "details": validation_errors
            }))
        ));
    }

    // Sanitize and validate
    if let Err(e) = model::validation::validate_reasoning_request(&mut req) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "Input validation failed",
                "details": e.to_string()
            }))
        ));
    }

    match call_local_model(&req) {
        Ok(res) => Ok((StatusCode::OK, Json(res))),
        Err(err) => Ok((
            StatusCode::BAD_REQUEST,
            Json(ReasoningResponse {
                summary: format!("Local model error: {err}"),
                ..ReasoningResponse::default()
            }),
        )),
    }
}

/// Basic verification of reasoning output
pub async fn verify(Json(mut resp): Json<ReasoningResponse>) -> Result<Json<VerifyResponse>, (StatusCode, Json<serde_json::Value>)> {
    // Validate input
    if let Err(validation_errors) = resp.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "Validation failed",
                "details": validation_errors
            }))
        ));
    }

    let valid = !resp.summary.trim().is_empty() && resp.summary.len() < 100000;
    let message = if valid {
        "Response appears valid".to_string()
    } else if resp.summary.trim().is_empty() {
        "Empty reasoning response".to_string()
    } else {
        "Response too long".to_string()
    };
    
    Ok(Json(VerifyResponse {
        valid,
        reason: message,
    }))
}

/// Store reasoning logs locally
pub async fn store(Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Validate payload size
    let payload_str = serde_json::to_string(&payload).unwrap_or_default();
    if payload_str.len() > 1_000_000 { // 1MB limit
        return Err((
            StatusCode::PAYLOAD_TOO_LARGE,
            Json(json!({
                "error": "Payload too large",
                "max_size": "1MB"
            }))
        ));
    }

    let id = uuid::Uuid::new_v4();
    let timestamp = time::OffsetDateTime::now_utc();

    let entry = LogEntry {
        id,
        timestamp,
        event: "store".into(),
        payload,
    };

    match tokio::fs::write(
        format!("/logs/{}.json", id),
        serde_json::to_string_pretty(&entry).unwrap(),
    )
    .await {
        Ok(_) => Ok(Json(json!({ "status": "stored", "id": id }))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Failed to store log entry",
                "details": e.to_string()
            }))
        )),
    }
}
