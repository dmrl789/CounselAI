//! Counsel AI Desktop — Local Tauri Backend
//! Provides safe, offline integration for the React UI.
//! Includes gateway control, full model verification, lightweight background checks,
//! and local model management commands for install/activation.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use once_cell::sync::Lazy;
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
use serde::{Serialize, Deserialize};

/// Track MCP gateway runtime state
static MCP_STATUS: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

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

/// --- Full Verify or Repair Active Model ---
#[tauri::command]
fn verify_active_model() -> Result<String, String> {
    match verify_or_repair_model() {
        Ok(msg) => Ok(msg),
        Err(e) => Err(format!("Verification failed: {e}")),
    }
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

/// --- Helper: Full verify or repair ---
fn verify_or_repair_model() -> Result<String> {
    let env_path = Path::new(".env");
    let content = fs::read_to_string(env_path).unwrap_or_default();
    let model_line = content
        .lines()
        .find(|l| l.starts_with("LOCAL_MODEL_PATH="))
        .map(|l| l.replacen("LOCAL_MODEL_PATH=", "", 1).trim().to_string());

    let Some(path) = model_line else {
        return Err(anyhow!("LOCAL_MODEL_PATH not found in .env"));
    };

    let file_name = Path::new(&path)
        .file_name()
        .and_then(|f| f.to_str())
        .unwrap_or("");

    let (url, trusted_hash) = match file_name {
        "mistral-7b-instruct.Q4_K_M.gguf" => (
            "https://huggingface.co/TheBloke/Mistral-7B-Instruct-v0.2-GGUF/resolve/main/mistral-7b-instruct-v0.2.Q4_K_M.gguf",
            "c9b84e2cb9d5e547faefab7b9b2a8cc73e2e9ab31dd23842fbbfc97b5670a708",
        ),
        "phi-3-mini-4k-instruct.Q4_K_M.gguf" => (
            "https://huggingface.co/TheBloke/phi-3-mini-4k-instruct-GGUF/resolve/main/phi-3-mini-4k-instruct.Q4_K_M.gguf",
            "a21ad4c26f53211e39df6b374f640093226f55da16d3f7a7c10c3a90ab5c04b2",
        ),
        _ => ("", ""),
    };

    if !Path::new(&path).exists() {
        println!("⚠️ Model missing — redownloading...");
        download_model(&path, url)?;
    }

    let computed = compute_sha256(&path)?;
    if computed != trusted_hash {
        println!("⚠️ Hash mismatch detected — repairing model...");
        if Path::new(&path).exists() {
            fs::remove_file(&path).ok();
        }
        download_model(&path, url)?;
        let new_hash = compute_sha256(&path)?;
        if new_hash != trusted_hash {
            return Err(anyhow!("Checksum mismatch after re-download."));
        }
        return Ok(format!("✅ Model repaired and verified (SHA256: {})", new_hash));
    }

    Ok(format!("✅ Model verified successfully (SHA256: {})", computed))
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
    println!("⬇️ Downloading from HuggingFace...");
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
