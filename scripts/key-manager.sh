#!/usr/bin/env bash
# Counsel AI – Key & Registry Management Utility
# Requires: gpg, sha256sum

set -euo pipefail

REG_PATH="services/mcp-gateway/models/trusted_models.json"
KEY_DIR="keys"
mkdir -p "$KEY_DIR"

CURRENT_KEY="$KEY_DIR/current.asc"
ARCHIVE_DIR="$KEY_DIR/archive"
mkdir -p "$ARCHIVE_DIR"

timestamp() { date +"%Y-%m-%dT%H:%M:%S"; }

rotate_key() {
  echo "🔐 Generating new signing key..."
  gpg --batch --passphrase "" --quick-generate-key "CounselAI Key $(timestamp)" rsa4096 sign 1y
  NEW_FPR=$(gpg --list-keys --with-colons | awk -F: '/^fpr:/ {print $10}' | tail -1)
  gpg --armor --export "$NEW_FPR" > "$CURRENT_KEY"
  cp "$CURRENT_KEY" "$ARCHIVE_DIR/publickey_${NEW_FPR}.asc"
  echo "✅ New key generated and exported: $CURRENT_KEY"
  echo "Fingerprint: $NEW_FPR"
}

sign_registry() {
  echo "🖋️  Signing $REG_PATH ..."
  gpg --armor --detach-sign --output "${REG_PATH}.asc" "$REG_PATH"
  sha256sum "$REG_PATH" > "${REG_PATH}.sha256"
  echo "✅ Registry signed and hash recorded."
}

verify_registry() {
  echo "🔍 Verifying signature..."
  gpg --verify "${REG_PATH}.asc" "$REG_PATH" || exit 1
  echo "✅ PGP signature OK."
  echo "Checking SHA256..."
  sha256sum -c "${REG_PATH}.sha256"
}

case "$1" in
  rotate) rotate_key ;;
  sign)   sign_registry ;;
  verify) verify_registry ;;
  *)
    echo "Usage: $0 {rotate|sign|verify}"
    ;;
esac
