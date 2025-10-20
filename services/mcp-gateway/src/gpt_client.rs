use crate::model::{ReasoningRequest, ReasoningResponse};
use anyhow::Result;
use reqwest::Client;
use serde_json::json;

/// Calls GPT-5 through OpenAI API securely
pub async fn call_gpt(reason_req: &ReasoningRequest) -> Result<ReasoningResponse> {
    let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let model = std::env::var("GPT_MODEL").unwrap_or_else(|_| "gpt-5".to_string());

    let client = Client::new();

    let body = json!({
        "model": model,
        "messages": [
            {"role": "system", "content": "You are a legal reasoning assistant for Counsel AI. You must never include private or identifying data."},
            {"role": "user", "content": reason_req.prompt}
        ]
    });

    let res = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .await?;

    let value: serde_json::Value = res.json().await?;
    let content = value["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("No response.")
        .to_string();

    Ok(ReasoningResponse {
        summary: content,
        citations: vec![],
    })
}
