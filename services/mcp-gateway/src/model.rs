use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;
use validator::{Validate, ValidationError};
use std::collections::HashSet;
use utoipa::ToSchema;

/// --- Query and Reasoning Data Models ---

/// Incoming query from UI
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct QueryRequest {
    #[validate(length(min = 1, max = 10000, message = "Text must be between 1 and 10000 characters"))]
    pub text: String,
    #[serde(default)]
    #[validate(length(max = 10, message = "Maximum 10 files allowed"))]
    pub files: Option<Vec<String>>,
}

/// Request to GPT or local model
#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct ReasoningRequest {
    /// Cleaned and concatenated user prompt
    #[validate(length(min = 1, max = 50000, message = "Prompt must be between 1 and 50000 characters"))]
    pub prompt: String,
    /// Optional contextual segments (case law, doctrine)
    #[serde(default)]
    #[validate(length(max = 20, message = "Maximum 20 context segments allowed"))]
    pub context: Vec<String>,
}

/// Unified reasoning response structure
#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct ReasoningResponse {
    /// Summarized or reasoned output text
    #[validate(length(max = 100000, message = "Summary too long"))]
    pub summary: String,
    /// Optional cited references or sources
    #[serde(default)]
    #[validate(length(max = 100, message = "Maximum 100 citations allowed"))]
    pub citations: Vec<String>,
}

/// Verification result for model integrity
#[derive(Debug, Serialize, ToSchema)]
pub struct VerifyResponse {
    pub valid: bool,
    pub reason: String,
}

/// Log entry for stored operations (auditable)
#[derive(Debug, Serialize, ToSchema)]
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

/// Input sanitization and validation utilities
pub mod validation {
    use super::*;
    use regex::Regex;
    use std::collections::HashSet;

    /// Sanitize text input by removing potentially dangerous content
    pub fn sanitize_text(input: &str) -> String {
        // Remove control characters except newlines and tabs
        let cleaned = input
            .chars()
            .filter(|c| !c.is_control() || *c == '\n' || *c == '\t')
            .collect::<String>();
        
        // Remove excessive whitespace
        let re = Regex::new(r"\s+").unwrap();
        re.replace_all(&cleaned, " ").trim().to_string()
    }

    /// Validate file paths to prevent directory traversal
    pub fn validate_file_paths(paths: &[String]) -> Result<(), ValidationError> {
        for path in paths {
            if path.contains("..") || path.starts_with('/') || path.contains('\\') {
                return Err(ValidationError::new("Invalid file path"));
            }
        }
        Ok(())
    }

    /// Check for potentially malicious content
    pub fn check_malicious_content(input: &str) -> bool {
        let dangerous_patterns = [
            r"<script",
            r"javascript:",
            r"data:",
            r"vbscript:",
            r"onload\s*=",
            r"onerror\s*=",
        ];
        
        let re = Regex::new(&dangerous_patterns.join("|")).unwrap();
        re.is_match(input)
    }

    /// Validate and sanitize query request
    pub fn validate_query_request(req: &mut QueryRequest) -> Result<(), ValidationError> {
        // Sanitize text
        req.text = sanitize_text(&req.text);
        
        // Check for malicious content
        if check_malicious_content(&req.text) {
            return Err(ValidationError::new("Potentially malicious content detected"));
        }
        
        // Validate file paths if present
        if let Some(ref files) = req.files {
            validate_file_paths(files)?;
        }
        
        Ok(())
    }

    /// Validate reasoning request
    pub fn validate_reasoning_request(req: &mut ReasoningRequest) -> Result<(), ValidationError> {
        // Sanitize prompt
        req.prompt = sanitize_text(&req.prompt);
        
        // Check for malicious content
        if check_malicious_content(&req.prompt) {
            return Err(ValidationError::new("Potentially malicious content detected"));
        }
        
        // Sanitize context
        req.context = req.context
            .iter()
            .map(|c| sanitize_text(c))
            .collect();
        
        Ok(())
    }
}
