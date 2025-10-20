use crate::model::{ReasoningRequest, ReasoningResponse};
use anyhow::{Context, Result};
use llama_cpp::{Llama, LlamaOptions};
use reqwest::Client;
use serde_json::json;
use std::path::Path;

pub async fn call_gpt(reason_req: &ReasoningRequest) -> Result<ReasoningResponse> {
    if std::env::var("OPENAI_API_KEY").is_err() {
        return call_local_model(reason_req);
    }

    let api_key = std::env::var("OPENAI_API_KEY")
        .context("OPENAI_API_KEY is required for online reasoning")?;
    let model = std::env::var("GPT_MODEL").unwrap_or_else(|_| "gpt-5".to_string());

    let client = Client::new();
    let body = json!({
        "model": model,
        "messages": [
            {"role": "system", "content": "You are a legal reasoning assistant for Counsel AI."},
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

pub fn call_local_model(req: &ReasoningRequest) -> Result<ReasoningResponse> {
    let model_path = std::env::var("LOCAL_MODEL_PATH")
        .unwrap_or_else(|_| "./models/mistral-7b-instruct.Q4_K_M.gguf".to_string());
    let prompt = format!(
        "You are an Italian legal assistant. Summarize: {}",
        req.prompt
    );

    let mut llama = Llama::new_with_options(
        Path::new(&model_path),
        LlamaOptions::default().n_threads(4),
    )?;

    let response = llama.run(&prompt, None)?;
    Ok(ReasoningResponse {
        summary: response.trim().to_string(),
        citations: vec![],
    })
}
