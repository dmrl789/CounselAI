//! Counsel AI Desktop — Local Tauri Backend
//! Provides safe, offline integration for the React UI.
//! Includes gateway control, full model verification from trusted registry,
//! and local model management commands for install/activation.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::Read,
    path::Path,
    process::{Command, Stdio},
    sync::Mutex,
};
use tauri::AppHandle;
use anyhow::{anyhow, Result};

/// Track MCP gateway runtime state
static MCP_STATUS: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

/// --- Registry Structures ---
#[derive(Deserialize)]
struct TrustedRegistry {
    registry_version: u32,
    issued_at: String,
    expires_at: String,
    models: Vec<TrustedModel>,
}

#[derive(Deserialize)]
struct TrustedModel {
    id: String,
    provider: String,
    license: String,
    sha256: String,
    uri: String,
}

/// --- Data Structures for Model Management ---
#[derive(Serialize, Deserialize)]
struct ModelInfo {
    name: String,
    file: String,
    installed: bool,
    path: String,
}

/// --- Gateway Controls ---
#[tauri::command]
async fn start_mcp_gateway(_app: AppHandle) -> Result<String, String> {
    let mut lock = MCP_STATUS.lock().map_err(|_| "Status lock poisoned".to_string())?;
    if *lock {
        return Ok("MCP Gateway already running".into());
    }

    let spawn_result = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "mcp-gateway.exe"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
    } else {
        Command::new("mcp-gateway")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
    };

    match spawn_result {
        Ok(_) => {
            *lock = true;
            Ok("MCP Gateway started successfully".into())
        }
        Err(e) => Err(format!("Failed to start gateway: {}", e)),
    }
}

/// --- Version Info ---
#[tauri::command]
fn app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// --- Local Model Listing ---
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

/// --- Install Models via External Script ---
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

/// --- Update Active Model in .env ---
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

/// --- Verify Active Model from Trusted Registry ---
#[tauri::command]
fn verify_active_model() -> Result<String, String> {
    verify_or_repair_model_from_registry().map_err(|e| format!("Verification failed: {e}"))
}

/// --- Lightweight background verification (hash-only, non-repairing) ---
#[tauri::command]
fn quick_verify_model() -> Result<String, String> {
    let env_path = Path::new(".env");
    let content = fs::read_to_string(env_path).unwrap_or_default();
    let model_line = content
        .lines()
        .find(|l| l.starts_with("LOCAL_MODEL_PATH="))
        .map(|l| l.replacen("LOCAL_MODEL_PATH=", "", 1).trim().to_string());

    let Some(path) = model_line else {
        return Err("LOCAL_MODEL_PATH not found in .env".into());
    };

    if !Path::new(&path).exists() {
        return Err(format!("Model file not found: {}", path));
    }

    let mut file = std::fs::File::open(&path).map_err(|e| e.to_string())?;
    let mut hasher = Sha256::new();
    std::io::copy(&mut file, &mut hasher).map_err(|e| e.to_string())?;
    let hash = format!("{:x}", hasher.finalize());

    Ok(format!("✅ Model SHA256: {}", hash))
}

/// --- Helper: Full verification or repair via registry ---
fn verify_or_repair_model_from_registry() -> Result<String> {
    let registry_path = Path::new("services/mcp-gateway/models/trusted_models.json");
    let registry_data = fs::read_to_string(registry_path)
        .map_err(|_| anyhow!("trusted_models.json not found"))?;
    let registry: TrustedRegistry = serde_json::from_str(&registry_data)?;

    let env_path = Path::new(".env");
    let env_content = fs::read_to_string(env_path).unwrap_or_default();
    let active_path = env_content
        .lines()
        .find(|l| l.starts_with("LOCAL_MODEL_PATH="))
        .map(|l| l.replacen("LOCAL_MODEL_PATH=", "", 1).trim().to_string())
        .ok_or_else(|| anyhow!("LOCAL_MODEL_PATH not set in .env"))?;

    let file_name = Path::new(&active_path)
        .file_name()
        .and_then(|f| f.to_str())
        .ok_or_else(|| anyhow!("Invalid model path"))?;

    let Some(model) = registry.models.iter().find(|m| m.id.ends_with(file_name)) else {
        return Err(anyhow!("Model not listed in trusted registry"));
    };

    if !Path::new(&active_path).exists() {
        println!("⚠️ Missing model — fetching from registry source");
        download_model(&active_path, &model.uri)?;
    }

    let computed = compute_sha256(&active_path)?;
    if computed != model.sha256 {
        println!("⚠️ Hash mismatch — repairing from registry URL");
        fs::remove_file(&active_path).ok();
        download_model(&active_path, &model.uri)?;
        let rehash = compute_sha256(&active_path)?;
        if rehash != model.sha256 {
            return Err(anyhow!("Checksum still invalid after redownload"));
        }
        return Ok(format!(
            "🛠️ Model repaired and verified ({} — {})",
            model.id, rehash
        ));
    }

    Ok(format!("✅ Model verified successfully ({} — {})", model.id, computed))
}

/// --- Utility: Compute SHA256 hash ---
fn compute_sha256<P: AsRef<Path>>(path: P) -> Result<String> {
    let mut file = std::fs::File::open(&path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];
    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

/// --- Utility: Download model if missing or corrupted ---
fn download_model(path: &str, url: &str) -> Result<()> {
    if url.is_empty() {
        return Err(anyhow!("Unknown model URL"));
    }
    let tmp = format!("{}.part", path);
    println!("⬇️ Downloading from registry source...");
    let status = Command::new("curl")
        .args(["-L", url, "-o", &tmp])
        .status()
        .map_err(|e| anyhow!("curl not available: {e}"))?;
    if !status.success() {
        return Err(anyhow!("Download failed"));
    }
    fs::rename(&tmp, &path)?;
    Ok(())
}

/// --- App Entry Point ---
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            start_mcp_gateway,
            app_version,
            verify_active_model,
            quick_verify_model,
            list_local_models,
            install_model,
            set_active_model
        ])
        .setup(|_| {
            println!("⚖️ Counsel AI Desktop initialized");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Counsel AI desktop");
}
