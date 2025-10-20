#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    fs,
    io::Read,
    path::Path,
    process::Command,
};

use anyhow::{anyhow, Result};
use sha2::{Digest, Sha256};

type StdResult<T, E> = std::result::Result<T, E>;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![verify_active_model])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn verify_active_model() -> StdResult<String, String> {
    match verify_or_repair_model() {
        Ok(msg) => Ok(msg),
        Err(e) => Err(format!("Verification failed: {e}")),
    }
}

fn verify_or_repair_model() -> Result<String> {
    let env_path = Path::new(".env");
    let content = fs::read_to_string(env_path).unwrap_or_default();
    let model_line = content
        .lines()
        .find(|l| l.starts_with("LOCAL_MODEL_PATH="))
        .map(|l| l.replace("LOCAL_MODEL_PATH=", "").trim().to_string());

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
        return Ok(format!(
            "✅ Model repaired and verified (SHA256: {})",
            new_hash
        ));
    }

    Ok(format!(
        "✅ Model verified successfully (SHA256: {})",
        computed
    ))
}

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
