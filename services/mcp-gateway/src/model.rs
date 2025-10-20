use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

/// Basic query structure from the UI
#[derive(Debug, Deserialize)]
pub struct QueryRequest {
    pub text: String,
    pub files: Option<Vec<String>>,
}

/// Sanitized reasoning bundle
#[derive(Debug, Serialize, Deserialize)]
pub struct ReasoningRequest {
    pub prompt: String,
    pub context: Vec<String>,
}

/// Response from GPT-5
#[derive(Debug, Serialize, Deserialize)]
pub struct ReasoningResponse {
    pub summary: String,
    pub citations: Vec<String>,
}

/// Verification result
#[derive(Debug, Serialize)]
pub struct VerifyResponse {
    pub valid: bool,
    pub reason: String,
}

/// Stored log entry
#[derive(Debug, Serialize)]
pub struct LogEntry {
    pub id: Uuid,
    pub timestamp: OffsetDateTime,
    pub event: String,
    pub payload: serde_json::Value,
}
