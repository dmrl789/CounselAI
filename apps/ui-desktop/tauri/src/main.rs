use serde::Deserialize;
use std::{fs, path::Path};
use sha2::{Digest, Sha256};
use anyhow::{Result, anyhow};

#[derive(Deserialize)]
struct TrustedRegistry {
    version: u32,
    updated: String,
    models: Vec<TrustedModel>,
}

#[derive(Deserialize)]
struct TrustedModel {
    id: String,
    name: String,
    file: String,
    url: String,
    sha256: String,
    trusted: bool,
}

#[tauri::command]
fn verify_active_model() -> Result<String, String> {
    verify_or_repair_model_from_registry().map_err(|e| e.to_string())
}

fn verify_or_repair_model_from_registry() -> Result<String> {
    let registry_path = Path::new("services/mcp-gateway/models/trusted_models.json");
    let data = fs::read_to_string(registry_path)
        .map_err(|_| anyhow!("trusted_models.json not found"))?;
    let registry: TrustedRegistry = serde_json::from_str(&data)?;

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

    let Some(model) = registry.models.iter().find(|m| m.file == file_name) else {
        return Err(anyhow!("Model not listed in trusted registry"));
    };

    if !model.trusted {
        return Err(anyhow!("Model marked untrusted in registry"));
    }

    if !Path::new(&active_path).exists() {
        println!("⚠️ Missing model — fetching from registry source");
        download_model(&active_path, &model.url)?;
    }

    let computed = compute_sha256(&active_path)?;
    if computed != model.sha256 {
        println!("⚠️ Hash mismatch — repairing from registry URL");
        fs::remove_file(&active_path).ok();
        download_model(&active_path, &model.url)?;
        let rehash = compute_sha256(&active_path)?;
        if rehash != model.sha256 {
            return Err(anyhow!("Checksum still invalid after redownload"));
        }
        return Ok(format!(
            "🛠️ Model repaired and verified ({} — {})",
            model.name, rehash
        ));
    }

    Ok(format!(
        "✅ Model verified ({} — {})",
        model.name, computed
    ))
}

fn compute_sha256<P: AsRef<Path>>(path: P) -> Result<String> {
    let mut f = std::fs::File::open(path)?;
    let mut hasher = Sha256::new();
    std::io::copy(&mut f, &mut hasher)?;
    Ok(format!("{:x}", hasher.finalize()))
}

fn download_model(path: &str, url: &str) -> Result<()> {
    let tmp = format!("{}.part", path);
    let status = std::process::Command::new("curl")
        .args(["-L", url, "-o", &tmp])
        .status()
        .map_err(|e| anyhow!("curl error: {e}"))?;
    if !status.success() {
        return Err(anyhow!("Download failed"));
    }
    fs::rename(&tmp, path)?;
    Ok(())
}
