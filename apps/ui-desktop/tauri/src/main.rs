//! Counsel AI Desktop — Local Tauri Backend
//! Provides safe, offline integration for the React UI.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use once_cell::sync::Lazy;
use std::process::{Command, Stdio};
use std::sync::Mutex;
use tauri::AppHandle;

// Track MCP gateway status (optional)
static MCP_STATUS: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

#[tauri::command]
async fn start_mcp_gateway(_app: AppHandle) -> Result<String, String> {
  // Attempt to start the MCP Gateway binary (if installed locally)
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

#[tauri::command]
fn app_version() -> String {
  env!("CARGO_PKG_VERSION").to_string()
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![start_mcp_gateway, app_version])
    .setup(|_| {
      println!("⚖️ Counsel AI Desktop initialized");
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running Counsel AI desktop");
}
