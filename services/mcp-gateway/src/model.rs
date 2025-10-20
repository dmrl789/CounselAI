use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

/// --- Query and Reasoning Data Models ---

/// Incoming query from UI
#[derive(Debug, Deserialize)]
pub struct QueryRequest {
    pub text: String,
    #[serde(default)]
    pub files: Option<Vec<String>>,
}

/// Request to GPT or local model
#[derive(Debug, Serialize, Deserialize)]
pub struct ReasoningRequest {
    /// Cleaned and concatenated user prompt
    pub prompt: String,
    /// Optional contextual segments (case law, doctrine)
    #[serde(default)]
    pub context: Vec<String>,
}

/// Unified reasoning response structure
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ReasoningResponse {
    /// Summarized or reasoned output text
    pub summary: String,
    /// Optional cited references or sources
    #[serde(default)]
    pub citations: Vec<String>,
}

/// Verification result for model integrity
#[derive(Debug, Serialize)]
pub struct VerifyResponse {
    pub valid: bool,
    pub reason: String,
}

/// Log entry for stored operations (auditable)
#[derive(Debug, Serialize)]
pub struct LogEntry {
    pub id: Uuid,
    pub timestamp: OffsetDateTime,
    pub event: String,
    pub payload: serde_json::Value,
}

impl Default for ReasoningResponse {
    fn default() -> Self {
        Self {
            summary: String::new(),
            citations: vec![],
        }
    }
}
