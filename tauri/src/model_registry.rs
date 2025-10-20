use std::path::{Path, PathBuf};
use std::fs;

use anyhow::{anyhow, Context, Result};
use pgp::{composed::SignedPublicKey, SignedMessage, Deserializable};

const REGISTRY_DIR: &str = "../services/mcp-gateway/models";
const REGISTRY_FILE: &str = "trusted_models.json";
const SIGNATURE_FILE: &str = "trusted_models.json.asc";
const PUBLIC_KEY_FILE: &str = "publickey.asc";

fn default_registry_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(REGISTRY_DIR)
}

fn read_file(path: &Path) -> Result<Vec<u8>> {
    fs::read(path).with_context(|| format!("failed to read {}", path.display()))
}

fn parse_public_key(bytes: &[u8]) -> Result<SignedPublicKey> {
    let (key, _keys) = SignedPublicKey::from_armor_single(bytes)
        .map_err(|_| anyhow!("invalid public key format"))?;
    Ok(key)
}

fn parse_signature(bytes: &[u8]) -> Result<SignedMessage> {
    let (sig, _rest) = SignedMessage::from_armor_single(bytes)
        .map_err(|_| anyhow!("invalid signature file"))?;
    Ok(sig)
}

pub fn verify_registry_signature_with_paths<P: AsRef<Path>, Q: AsRef<Path>, R: AsRef<Path>>(
    public_key_path: P,
    registry_path: Q,
    signature_path: R,
) -> Result<()> {
    let public_key_bytes = read_file(public_key_path.as_ref())?;
    let registry_bytes = read_file(registry_path.as_ref())?;
    let signature_bytes = read_file(signature_path.as_ref())?;

    let public_key = parse_public_key(&public_key_bytes)?;
    let signature = parse_signature(&signature_bytes)?;

    signature
        .verify(&public_key, &registry_bytes)
        .map_err(|_| anyhow!("signature verification failed"))?;

    Ok(())
}

pub fn verify_registry_signature() -> Result<()> {
    let dir = default_registry_dir();
    verify_registry_signature_with_paths(
        dir.join(PUBLIC_KEY_FILE),
        dir.join(REGISTRY_FILE),
        dir.join(SIGNATURE_FILE),
    )
}

pub fn verify_or_repair_model_from_registry() -> Result<()> {
    verify_registry_signature()?;
    // existing logic would follow...
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    fn fixture_paths() -> (PathBuf, PathBuf, PathBuf) {
        let base = default_registry_dir();
        (
            base.join(PUBLIC_KEY_FILE),
            base.join(REGISTRY_FILE),
            base.join(SIGNATURE_FILE),
        )
    }

    #[test]
    fn signature_verifies_for_pristine_registry() {
        let (pubkey, registry, signature) = fixture_paths();
        verify_registry_signature_with_paths(pubkey, registry, signature).unwrap();
    }

    #[test]
    fn tampering_breaks_signature() {
        let (pubkey, registry, signature) = fixture_paths();
        let original = read_file(&registry).unwrap();
        let mut tampered = original.clone();
        if let Some(first) = tampered.first_mut() {
            *first ^= 0xFF;
        }

        let mut temp = NamedTempFile::new().unwrap();
        temp.write_all(&tampered).unwrap();
        let temp_path = temp.into_temp_path();

        let result = verify_registry_signature_with_paths(pubkey, &temp_path, signature);
        assert!(result.is_err());
    }
}
