use axum::Json;
use serde_json::json;

use crate::gpt_client::call_gpt;
use crate::model::*;

pub async fn health() -> &'static str {
    "ok"
}

pub async fn query(Json(req): Json<QueryRequest>) -> Json<ReasoningRequest> {
    // Simple sanitization placeholder
    let sanitized = req.text.replace(|c: char| c.is_ascii_punctuation(), "");
    let reason_req = ReasoningRequest {
        prompt: format!("Summarize and reason about: {}", sanitized),
        context: vec![],
    };
    Json(reason_req)
}

pub async fn reason(Json(reason_req): Json<ReasoningRequest>) -> Json<ReasoningResponse> {
    match call_gpt(&reason_req).await {
        Ok(res) => Json(res),
        Err(e) => Json(ReasoningResponse {
            summary: format!("Error: {e}"),
            citations: vec![],
        }),
    }
}

pub async fn verify(Json(resp): Json<ReasoningResponse>) -> Json<VerifyResponse> {
    let valid = !resp.summary.is_empty();
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
    Json(json!({"status": "stored", "id": id}))
}
