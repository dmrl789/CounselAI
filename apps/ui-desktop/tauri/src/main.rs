#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sha2::{Digest, Sha256};
use std::{
    fs::{self, File},
    io::Read,
    path::Path,
};

#[tauri::command]
fn list_local_models() -> Result<Vec<String>, String> {
    Err("list_local_models not implemented".into())
}

#[tauri::command]
fn install_model(_model_name: String) -> Result<(), String> {
    Err("install_model not implemented".into())
}

#[tauri::command]
fn set_active_model(_model_name: String) -> Result<(), String> {
    Err("set_active_model not implemented".into())
}

#[tauri::command]
fn verify_active_model() -> Result<String, String> {
    let env_path = Path::new(".env");
    let content = fs::read_to_string(env_path).unwrap_or_default();
    let model_line = content
        .lines()
        .find(|l| l.starts_with("LOCAL_MODEL_PATH="))
        .map(|l| l.replace("LOCAL_MODEL_PATH=", "").trim().to_string());

    let Some(path) = model_line else {
        return Err("LOCAL_MODEL_PATH not found in .env".into());
    };

    if !Path::new(&path).exists() {
        return Err(format!("Model file not found: {}", path));
    }

    let mut file = File::open(&path).map_err(|e| e.to_string())?;
    let mut hasher = Sha256::new();
    std::io::copy(&mut file, &mut hasher).map_err(|e| e.to_string())?;
    let hash = format!("{:x}", hasher.finalize());

    Ok(hash)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            list_local_models,
            install_model,
            set_active_model,
            verify_active_model
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
