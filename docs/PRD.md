# Counsel AI — Product Requirements Document  
**Version 1.3** — Updated 20 Oct 2025  
**Maintainers:** D. Verga / U. Giuliani  
**Status:** Internal Engineering Baseline  

---

## 1. Mission & Vision
Counsel AI is a secure, locally executed legal-reasoning agent that assists Italian lawyers in drafting, analyzing, and validating legal opinions.  
It operates through a **local Model Context Protocol (MCP)** that guarantees *zero data leakage* and maintains a verifiable chain-of-trust from data ingestion to model inference.

---

## 2. Core Principles
| Principle | Description |
|------------|-------------|
| **Full Privacy** | All client data processed and stored locally; GPT calls receive only sanitized text. |
| **Deterministic Reasoning** | Legal logic derived from codified sources, doctrine, and precedents; results reproducible. |
| **Provable Integrity** | Every reasoning artifact and model verified by cryptographic hash and PGP signature. |
| **Self-Healing** | Damaged or missing models automatically re-downloaded and re-verified from trusted registry. |
| **Offline Resilience** | When internet or API key unavailable, Counsel AI uses local LLM (Mistral 7B / Phi-3) via `llama.cpp`. |

---

## 3. System Architecture
```

┌────────────────────────────┐
│  Desktop UI (Tauri + React)│
│  • Chat & File upload       │
│  • Model Manager / Verifier │
└─────────────┬──────────────┘
│
┌─────────────▼──────────────┐
│  Local MCP Gateway (Rust)  │
│  • Context sanitization     │
│  • /query / reason / verify │
│  • Offline fallback (LLM)   │
│  • HashTimer timestamping   │
└─────────────┬──────────────┘
│
┌─────────────▼──────────────┐
│  GPT-5 Reasoner (Cloud)    │
│  *Stateless API*           │
└─────────────┬──────────────┘
│
┌─────────────▼──────────────┐
│  Legal KB + Vector Store   │
│  (Qdrant / SQLite)         │
└────────────────────────────┘

```

---

## 4. Key Modules

| Module | Technology | Responsibilities |
|---------|-------------|------------------|
| **MCP Gateway** | Rust + Axum | Request routing, context redaction, GPT bridge, offline LLM execution. |
| **UI Desktop** | React + Tauri | Chat interface, file ingestion, model controls, verification feedback. |
| **Model Manager** | Rust CLI / Tauri API | Download / verify / activate GGUF models. |
| **Model Verifier** | Rust + SHA-256 | Auto-check integrity, repair from trusted URL. |
| **Trusted Registry** | JSON + PGP signature | Source-of-truth for model URLs & hashes. |
| **Key Manager** | Bash + GPG | Rotate signing keys, sign registry, verify hashes. |

---

## 5. Data Flow & Privacy
| Step | Process | Exposure |
|------|----------|----------|
| 1 | User uploads or types legal text | Local |
| 2 | MCP sanitizes, vectorizes, stores | Local |
| 3 | Sanitized context → GPT-5 API | Non-identifiable |
| 4 | Result returned → reinjected context | Local |
| 5 | Logs & opinions stored AES-256-encrypted | Local |

---

## 6. Model Trust Pipeline
| Stage | Mechanism | Description |
|--------|------------|-------------|
| **Registry** | `trusted_models.json` | Lists known-good models (URL + SHA-256 + license). |
| **Signature** | `trusted_models.json.asc` + PGP key | Must verify before use. |
| **Verification** | SHA-256 check per launch | Confirms model integrity. |
| **Auto-Repair** | `curl + hash` | Redownloads missing or corrupt model. |
| **Key Rotation** | `key-manager.sh rotate` | Annual signing key renewal. |
| **Audit Trail** | Archived keys + hashes | Provenance of every registry version. |

---

## 7. Security Controls
- **AES-256-GCM** encryption for all client data.  
- **libsodium** for signatures and random nonce generation.  
- **PGP signature enforcement** on registry load.  
- **Automatic expiry rejection** for unsigned or >180 days-old registries.  
- **Local-only file and network permissions** (Tauri allowlist = none external).  

---

## 8. Offline Operation
| Condition | Engine | Notes |
|------------|---------|-------|
| `OPENAI_API_KEY` present | GPT-5 cloud | Sanitized reasoning. |
| Absent / offline | Local LLM via `llama.cpp` | Mistral 7B Instruct or Phi-3 Mini GGUF. |
| Model corrupted | Auto-repair from registry | Verified via hash & PGP. |

---

## 9. Deployment
- **Docker Compose (local):** MCP Gateway + Qdrant + UI.  
- **Tauri Desktop binary:** Windows, macOS, Linux.  
- **Data Dirs:**  
  - `/data/encrypted/` — opinions + logs  
  - `/models/` — GGUF models  
  - `/keys/` — current & archived PGP keys  

---

## 10. Compliance & Audit
- **GDPR / MiFID / ISO-27001** aligned processing.  
- **Audit Record:** every reasoning call hashed + timestamped (HashTimer™).  
- **Local Retention Policy:** configurable purge after 90 days.  

---

## 11. Performance Targets
| Metric | Target |
|---------|---------|
| Response latency (GPT-5) | < 2 s |
| Response latency (local LLM) | < 6 s |
| Registry verification time | < 300 ms |
| Model auto-repair window | < 5 min on 4 GB model |
| Max file size ingest | 25 MB (PDF/DOCX) |

---

## 12. Future Extensions
- Multi-jurisdiction law modules (EU / ECHR / GDPR).  
- zk-STARK proofs for document existence and timestamping.  
- Distributed Registry signing via FinDAG nodes.  
- Voice dictation interface (local Whisper).  
- Smart contract drafting plug-in.  

---

## 13. Acceptance Criteria
- ✅ All network calls pass through MCP Gateway.  
- ✅ No external data stored outside local environment.  
- ✅ Registry signature verification must succeed or abort load.  
- ✅ Local LLM runs deterministically and logs hash outputs.  

---

**End of Document**
