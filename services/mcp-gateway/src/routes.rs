use axum::{http::StatusCode, Json};
use serde_json::json;

use crate::{
    gpt_client::{call_gpt, call_local_model},
    model::*,
};

/// Basic health endpoint
pub async fn health() -> &'static str {
    "ok"
}

/// Input sanitization & preprocessing step
pub async fn query(Json(req): Json<QueryRequest>) -> Json<ReasoningRequest> {
    // Basic sanitization placeholder (removes punctuation)
    let sanitized = req.text.replace(|c: char| c.is_ascii_punctuation(), "");
    let reason_req = ReasoningRequest {
        prompt: format!("Summarize and reason about: {}", sanitized),
        context: vec![],
    };
    Json(reason_req)
}

/// Online reasoning via GPT-5 (preferred if API key available)
pub async fn reason(Json(reason_req): Json<ReasoningRequest>) -> (StatusCode, Json<ReasoningResponse>) {
    match call_gpt(&reason_req).await {
        Ok(res) => (StatusCode::OK, Json(res)),
        Err(err) => (
            StatusCode::BAD_GATEWAY,
            Json(ReasoningResponse {
                summary: format!("Online reasoning error: {err}"),
                ..ReasoningResponse::default()
            }),
        ),
    }
}

/// Offline reasoning via local llama.cpp model (fallback)
pub async fn reason_local(Json(req): Json<ReasoningRequest>) -> (StatusCode, Json<ReasoningResponse>) {
    match call_local_model(&req) {
        Ok(res) => (StatusCode::OK, Json(res)),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(ReasoningResponse {
                summary: format!("Local model error: {err}"),
                ..ReasoningResponse::default()
            }),
        ),
    }
}

/// Basic verification of reasoning output
pub async fn verify(Json(resp): Json<ReasoningResponse>) -> Json<VerifyResponse> {
    let valid = !resp.summary.trim().is_empty();
    let message = if valid {
        "Response appears valid".to_string()
    } else {
        "Empty reasoning response".to_string()
    };
    Json(VerifyResponse {
        valid,
        reason: message,
    })
}

/// Store reasoning logs locally
pub async fn store(Json(payload): Json<serde_json::Value>) -> Json<serde_json::Value> {
    let id = uuid::Uuid::new_v4();
    let timestamp = time::OffsetDateTime::now_utc();

    let entry = LogEntry {
        id,
        timestamp,
        event: "store".into(),
        payload,
    };

    let _ = tokio::fs::write(
        format!("/logs/{}.json", id),
        serde_json::to_string_pretty(&entry).unwrap(),
    )
    .await;

    Json(json!({ "status": "stored", "id": id }))
}
