# **Counsel AI — Product Requirements Document**  
**Version 1.3** — Updated 20 Oct 2025  
**Maintainers:** D. Verga / U. Giuliani  
**Status:** Internal Engineering Baseline  

---

## **1. Mission & Vision**
Counsel AI is a secure, locally executed **legal-reasoning agent** assisting Italian lawyers in drafting, analyzing, and validating legal opinions.  
It operates through a **local Model Context Protocol (MCP)** that guarantees *zero data leakage* and maintains a verifiable chain-of-trust from data ingestion to model inference.

---

## **2. Core Principles**

| Principle | Description |
|------------|-------------|
| **Full Privacy** | All data processed and stored locally; GPT calls receive only sanitized text. |
| **Deterministic Reasoning** | Legal logic derived from codes, doctrine, and precedents; results reproducible. |
| **Provable Integrity** | Every reasoning artifact + model verified via SHA-256 hash + PGP signature. |
| **Self-Healing** | Damaged or missing models automatically re-downloaded and re-verified. |
| **Offline Resilience** | When offline or no API key, Counsel AI uses local LLM (Mistral 7B / Phi-3 GGUF via `llama.cpp`). |

---

## **3. System Architecture**

```mermaid
flowchart TD
  %% === UI LAYER ===
  subgraph UI["🖥️ Counsel AI Desktop UI (Tauri + React)"]
    U1[Chat Interface & File Uploader]
    U2[Model Manager + Verifier Panel]
    U3[Local Encrypted Storage]
    U1 --> |Query JSON| M1
    U2 --> |Model Selection| M1
  end

  %% === MCP LAYER ===
  subgraph MCP["⚙️ Local MCP Gateway (Rust + Axum)"]
    M1[/Context Sanitizer + Router/]
    M2[/Reasoner + Offline Fallback/]
    M3[/Verifier + Logger/]
    M1 --> M2 --> M3
  end

  %% === MODEL LAYER ===
  subgraph MODELS["🧠 Reasoning Engines"]
    C1[GPT-5 Cloud Reasoner]
    C2[Local LLM (Mistral 7B / Phi-3 GGUF)]
  end
  M2 --> |API if key present| C1
  M2 --> |Offline Fallback| C2

  %% === TRUST LAYER ===
  subgraph TRUST["🔒 Trust & Verification Subsystem"]
    R1[trusted_models.json 📜]
    R2[PGP Signature (trusted_models.json.asc)]
    R3[Public Key (keys/current.asc)]
    R4[Key Manager 🔑 rotate / archive]
    R5[Model Verifier (SHA-256 + Auto-Repair)]
    R1 --> R2 --> R3 --> R4 --> R5
  end
  TRUST --> MCP

  %% === STORAGE LAYER ===
  subgraph DATA["🗄️ Local Storage & Vector DB"]
    D1[Encrypted Opinions / Logs]
    D2[Qdrant / SQLite Legal KB]
  end
  MCP --> DATA

  %% === FLOW LABELS ===
  U1 -. Sanitized Query .-> M1
  M3 -. Verified Response .-> U1
  TRUST -. Hashes + Keys .-> M3
  C1 -. Cloud Reasoning .-> M3
  C2 -. Local Inference .-> M3
