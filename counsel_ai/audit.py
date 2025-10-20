from __future__ import annotations

import json
import logging
from dataclasses import dataclass
from datetime import datetime
from hashlib import sha256
from pathlib import Path

logger = logging.getLogger(__name__)

AUDIT_DIR = Path.cwd() / "audit"
LEDGER_FILE = AUDIT_DIR / "ledger.jsonl"


@dataclass
class AuditRecord:
    timestamp: str
    action: str
    data_hash: str
    prev_hash: str


def _ensure_dir() -> None:
    AUDIT_DIR.mkdir(parents=True, exist_ok=True)


def _hash(data: str) -> str:
    return sha256(data.encode("utf-8")).hexdigest()


def append_record(action: str, payload: dict) -> str:
    """Append record to audit ledger with comprehensive error handling"""
    try:
        logger.info(f"Appending audit record for action: {action}")

        _ensure_dir()
        now = datetime.utcnow().isoformat()
        raw = json.dumps({"action": action, "payload": payload}, sort_keys=True)
        data_hash = _hash(raw)

        # Get previous hash from last record
        prev_hash = "0" * 64
        if LEDGER_FILE.exists():
            try:
                with LEDGER_FILE.open("r", encoding="utf-8") as f:
                    lines = f.read().strip().splitlines()
                    if lines:
                        last_line = lines[-1]
                        if last_line.strip():
                            prev = json.loads(last_line)
                            prev_hash = prev.get("chain_hash", prev_hash)
            except (json.JSONDecodeError, IOError) as e:
                logger.warning(f"Error reading previous hash: {e}. Using default.")

        chain_hash = _hash(prev_hash + data_hash)
        rec = {
            "timestamp": now,
            "action": action,
            "data_hash": data_hash,
            "prev_hash": prev_hash,
            "chain_hash": chain_hash,
            "payload": payload,
        }

        # Write record atomically
        temp_file = LEDGER_FILE.with_suffix(".tmp")
        try:
            # Read existing content and append new record
            existing_content = ""
            if LEDGER_FILE.exists():
                with LEDGER_FILE.open("r", encoding="utf-8") as f:
                    existing_content = f.read()

            with temp_file.open("w", encoding="utf-8") as f:
                f.write(existing_content)
                f.write(json.dumps(rec) + "\n")

            # Atomic move
            temp_file.replace(LEDGER_FILE)
            logger.info(f"Successfully appended audit record with hash: {chain_hash}")
            return chain_hash

        except Exception as e:
            # Clean up temp file on error
            if temp_file.exists():
                temp_file.unlink()
            raise

    except Exception as e:
        logger.error(f"Failed to append audit record: {e}", exc_info=True)
        raise


def verify_ledger_integrity() -> bool:
    """Verify the integrity of the audit ledger"""
    try:
        if not LEDGER_FILE.exists():
            logger.info("No ledger file exists - integrity verified")
            return True

        with LEDGER_FILE.open("r", encoding="utf-8") as f:
            lines = f.read().strip().splitlines()

        if not lines:
            logger.info("Empty ledger file - integrity verified")
            return True

        prev_hash = "0" * 64
        for i, line in enumerate(lines):
            if not line.strip():
                continue

            try:
                record = json.loads(line)
                expected_hash = _hash(prev_hash + record["data_hash"])

                if record["chain_hash"] != expected_hash:
                    logger.error(f"Hash mismatch at line {i+1}")
                    return False

                prev_hash = record["chain_hash"]

            except (json.JSONDecodeError, KeyError) as e:
                logger.error(f"Invalid record at line {i+1}: {e}")
                return False

        logger.info("Ledger integrity verified successfully")
        return True

    except Exception as e:
        logger.error(f"Error verifying ledger integrity: {e}", exc_info=True)
        return False
