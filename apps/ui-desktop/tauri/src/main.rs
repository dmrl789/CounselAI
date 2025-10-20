use serde::{Deserialize, Serialize};
use std::{fs, path::Path, process::Command};

#[derive(Serialize, Deserialize)]
struct ModelInfo {
    name: String,
    file: String,
    installed: bool,
    path: String,
}

#[tauri::command]
fn list_local_models() -> Vec<ModelInfo> {
    vec![
        ModelInfo {
            name: "Mistral-7B-Instruct-Q4_K_M".into(),
            file: "mistral-7b-instruct.Q4_K_M.gguf".into(),
            installed: Path::new("services/mcp-gateway/models/mistral-7b-instruct.Q4_K_M.gguf")
                .exists(),
            path: "services/mcp-gateway/models/mistral-7b-instruct.Q4_K_M.gguf".into(),
        },
        ModelInfo {
            name: "Phi-3-Mini-Instruct-Q4_K_M".into(),
            file: "phi-3-mini-4k-instruct.Q4_K_M.gguf".into(),
            installed: Path::new("services/mcp-gateway/models/phi-3-mini-4k-instruct.Q4_K_M.gguf")
                .exists(),
            path: "services/mcp-gateway/models/phi-3-mini-4k-instruct.Q4_K_M.gguf".into(),
        },
    ]
}

#[tauri::command]
async fn install_model(model_name: String) -> Result<String, String> {
    let status = Command::new("cargo")
        .args(["run", "--release", "--bin", "model-manager"])
        .status()
        .map_err(|e| format!("Failed to run model-manager: {e}"))?;
    if !status.success() {
        return Err("Model installation failed".into());
    }
    Ok(format!("✅ {} installed successfully", model_name))
}

#[tauri::command]
fn set_active_model(model_path: String) -> Result<String, String> {
    let env_path = Path::new(".env");
    let content = fs::read_to_string(env_path).unwrap_or_default();
    let updated = if content.contains("LOCAL_MODEL_PATH=") {
        content
            .lines()
            .map(|line| {
                if line.starts_with("LOCAL_MODEL_PATH=") {
                    format!("LOCAL_MODEL_PATH={}", model_path)
                } else {
                    line.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    } else if content.trim().is_empty() {
        format!("LOCAL_MODEL_PATH={}", model_path)
    } else {
        format!("{}\nLOCAL_MODEL_PATH={}", content, model_path)
    };
    fs::write(env_path, updated).map_err(|e| e.to_string())?;
    Ok(format!("Active model set to {}", model_path))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            list_local_models,
            install_model,
            set_active_model
        ])
        .run(tauri::generate_context!())
        .expect("error running Counsel AI desktop");
}
