use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ReasoningRequest {
    pub prompt: String,
}

#[derive(Debug, Serialize, Default)]
pub struct ReasoningResponse {
    pub summary: String,
    #[serde(default)]
    pub citations: Vec<String>,
}
