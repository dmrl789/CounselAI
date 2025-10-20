//! Counsel AI — Local Model Manager
//! Utility for downloading, verifying, and activating local LLMs.

use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

use anyhow::{Context, Result};
use sha2::{Digest, Sha256};

#[derive(Debug)]
struct Model {
    name: &'static str,
    url: &'static str,
    sha256: &'static str,
    file: &'static str,
}

const MODELS: &[Model] = &[
    Model {
        name: "Mistral-7B-Instruct-Q4_K_M",
        url: "https://huggingface.co/TheBloke/Mistral-7B-Instruct-v0.2-GGUF/resolve/main/mistral-7b-instruct-v0.2.Q4_K_M.gguf",
        sha256: "c9b84e2cb9d5e547faefab7b9b2a8cc73e2e9ab31dd23842fbbfc97b5670a708",
        file: "mistral-7b-instruct.Q4_K_M.gguf",
    },
    Model {
        name: "Phi-3-Mini-Instruct-Q4_K_M",
        url: "https://huggingface.co/TheBloke/phi-3-mini-4k-instruct-GGUF/resolve/main/phi-3-mini-4k-instruct.Q4_K_M.gguf",
        sha256: "a21ad4c26f53211e39df6b374f640093226f55da16d3f7a7c10c3a90ab5c04b2",
        file: "phi-3-mini-4k-instruct.Q4_K_M.gguf",
    },
];

fn main() -> Result<()> {
    println!("⚖️  Counsel AI Model Manager");
    println!("Available models:");
    for (i, m) in MODELS.iter().enumerate() {
        println!("  [{}] {} ", i + 1, m.name);
    }

    print!("Select model number to install: ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let choice: usize = input.trim().parse().unwrap_or(0);
    if choice == 0 || choice > MODELS.len() {
        println!("❌ Invalid choice");
        return Ok(());
    }

    let model = &MODELS[choice - 1];
    let models_dir = PathBuf::from("services/mcp-gateway/models");
    fs::create_dir_all(&models_dir)?;

    let model_path = models_dir.join(model.file);
    let tmp_path = model_path.with_extension("download");

    println!("⬇️  Downloading {} ...", model.name);
    let status = Command::new("curl")
        .args(["-L", model.url, "-o"])
        .arg(&tmp_path)
        .status()
        .expect("curl required");
    if !status.success() {
        anyhow::bail!("Download failed");
    }

    println!("🔍 Verifying SHA256 checksum ...");
    let mut file = File::open(&tmp_path)?;
    let mut hasher = Sha256::new();
    io::copy(&mut file, &mut hasher)?;
    let hash = format!("{:x}", hasher.finalize());

    if hash != model.sha256 {
        anyhow::bail!(
            "Checksum mismatch!\nExpected: {}\nFound:    {}",
            model.sha256,
            hash
        );
    }

    fs::rename(&tmp_path, &model_path)
        .with_context(|| "Failed to move model into place")?;
    println!("✅ Model verified and installed at {:?}", model_path);

    // Update .env
    let env_path = PathBuf::from(".env");
    let env_content = fs::read_to_string(&env_path).unwrap_or_default();
    let new_line = format!("LOCAL_MODEL_PATH={}\n", model_path.display());
    let updated = if env_content.contains("LOCAL_MODEL_PATH") {
        env_content
            .lines()
            .map(|l| {
                if l.starts_with("LOCAL_MODEL_PATH=") {
                    new_line.trim_end().to_string()
                } else {
                    l.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
            + "\n"
    } else {
        env_content + &new_line
    };
    fs::write(&env_path, updated)?;

    println!("🔧 .env updated → LOCAL_MODEL_PATH={}", model_path.display());
    println!("🎯 You can now restart the MCP Gateway to use {}", model.name);
    Ok(())
}
