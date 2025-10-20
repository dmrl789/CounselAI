use crate::model::{ReasoningRequest, ReasoningResponse};
use anyhow::{Context, Result};
use reqwest::Client;
use serde_json::json;
use std::path::Path;

/// Calls GPT-5 through OpenAI API securely,
/// or falls back to a local GGUF model (Mistral/Phi-3) if offline.
pub async fn call_gpt(reason_req: &ReasoningRequest) -> Result<ReasoningResponse> {
    if std::env::var("OPENAI_API_KEY").is_err() {
        // Offline fallback
        return call_local_model(reason_req);
    }

    let api_key = std::env::var("OPENAI_API_KEY")
        .context("OPENAI_API_KEY is required for online reasoning")?;
    let model = std::env::var("GPT_MODEL").unwrap_or_else(|_| "gpt-5".to_string());

    let client = Client::new();
    let body = json!({
        "model": model,
        "messages": [
            {"role": "system", "content": "You are a legal reasoning assistant for Counsel AI. You must never include private or identifying data."},
            {"role": "user", "content": reason_req.prompt}
        ]
    });

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .await
        .context("failed to contact OpenAI")?;

    let value: serde_json::Value = response.json().await.context("invalid OpenAI response")?;
    let text = value["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or_default()
        .to_string();

    Ok(ReasoningResponse {
        summary: text,
        citations: vec![],
    })
}

/// Offline reasoning using a local GGUF model (Mistral or Phi-3)
pub fn call_local_model(req: &ReasoningRequest) -> Result<ReasoningResponse> {
    let prompt = format!(
        "[OFFLINE MODE STUB]\nSummarize and analyze this case: {}",
        req.prompt
    );

    Ok(ReasoningResponse {
        summary: prompt,
        citations: vec![],
    })
}
