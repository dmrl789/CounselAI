from __future__ import annotations
from dataclasses import dataclass
from hashlib import sha256
from pathlib import Path
from datetime import datetime
import json

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
    _ensure_dir()
    now = datetime.utcnow().isoformat()
    raw = json.dumps({"action": action, "payload": payload}, sort_keys=True)
    data_hash = _hash(raw)
    prev_hash = "0" * 64
    if LEDGER_FILE.exists():
        *_, last = LEDGER_FILE.read_text().strip().splitlines() or [""]
        if last:
            prev = json.loads(last)
            prev_hash = prev.get("chain_hash", prev_hash)
    chain_hash = _hash(prev_hash + data_hash)
    rec = {
        "timestamp": now,
        "action": action,
        "data_hash": data_hash,
        "prev_hash": prev_hash,
        "chain_hash": chain_hash,
    }
    with LEDGER_FILE.open("a", encoding="utf-8") as f:
        f.write(json.dumps(rec) + "\n")
    return chain_hash
