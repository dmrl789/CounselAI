use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub bind_addr: String,
    pub api_key: String,
    pub openai_api_key: Option<String>,
    pub gpt_model: String,
    pub vector_db_url: String,
    pub storage_path: String,
    pub encryption_key: String,
    pub log_level: String,
    pub local_model_path: Option<String>,
    pub rate_limit_per_second: u32,
    pub rate_limit_burst_size: u32,
    pub max_request_size: usize,
    pub enable_compression: bool,
    pub enable_cors: bool,
}

impl Config {
    pub fn from_env() -> Result<Self, anyhow::Error> {
        Ok(Config {
            bind_addr: env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:5142".to_string()),
            api_key: env::var("API_KEY").unwrap_or_else(|_| "counsel-ai-dev-key".to_string()),
            openai_api_key: env::var("OPENAI_API_KEY").ok(),
            gpt_model: env::var("GPT_MODEL").unwrap_or_else(|_| "gpt-4".to_string()),
            vector_db_url: env::var("VECTOR_DB_URL").unwrap_or_else(|_| "http://localhost:6333".to_string()),
            storage_path: env::var("STORAGE_PATH").unwrap_or_else(|_| "/data/encrypted".to_string()),
            encryption_key: env::var("ENCRYPTION_KEY").unwrap_or_else(|_| "default-encryption-key-change-in-production".to_string()),
            log_level: env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
            local_model_path: env::var("LOCAL_MODEL_PATH").ok(),
            rate_limit_per_second: env::var("RATE_LIMIT_PER_SECOND")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .unwrap_or(10),
            rate_limit_burst_size: env::var("RATE_LIMIT_BURST_SIZE")
                .unwrap_or_else(|_| "20".to_string())
                .parse()
                .unwrap_or(20),
            max_request_size: env::var("MAX_REQUEST_SIZE")
                .unwrap_or_else(|_| "1048576".to_string()) // 1MB
                .parse()
                .unwrap_or(1048576),
            enable_compression: env::var("ENABLE_COMPRESSION")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
            enable_cors: env::var("ENABLE_CORS")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
        })
    }

    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.api_key.len() < 16 {
            errors.push("API_KEY must be at least 16 characters".to_string());
        }

        if self.encryption_key.len() < 32 {
            errors.push("ENCRYPTION_KEY must be at least 32 characters".to_string());
        }

        if self.rate_limit_per_second == 0 {
            errors.push("RATE_LIMIT_PER_SECOND must be greater than 0".to_string());
        }

        if self.rate_limit_burst_size == 0 {
            errors.push("RATE_LIMIT_BURST_SIZE must be greater than 0".to_string());
        }

        if self.max_request_size == 0 {
            errors.push("MAX_REQUEST_SIZE must be greater than 0".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}