use axum::{http::StatusCode, Json};

use crate::{
    gpt_client::{call_gpt, call_local_model},
    model::{ReasoningRequest, ReasoningResponse},
};

pub async fn reason(Json(req): Json<ReasoningRequest>) -> (StatusCode, Json<ReasoningResponse>) {
    match call_gpt(&req).await {
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

pub async fn reason_local(Json(req): Json<ReasoningRequest>) -> Json<ReasoningResponse> {
    match call_local_model(&req) {
        Ok(res) => Json(res),
        Err(err) => Json(ReasoningResponse {
            summary: format!("Local model error: {err}"),
            ..ReasoningResponse::default()
        }),
    }
}

pub async fn health() -> &'static str {
    "ok"
}
