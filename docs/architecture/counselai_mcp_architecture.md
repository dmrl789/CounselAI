# Counsel AI — Local MCP Architecture & Privacy Reasoning Layer

> **Version:** 1.0  
> **Author:** Désirée Verga / Ugo Giuliani  
> **Project:** Counsel AI — Italian Legal Reasoning Agent  
> **Goal:** Enable legal reasoning via GPT-5 with *zero data leakage* through a fully local **Model Context Protocol (MCP)**.

---

## 1. System Overview

Counsel AI is a secure legal assistant that drafts, reasons, and cites according to Italian law.  
Its reasoning engine (GPT-5) is isolated behind a **local MCP gateway** that feeds anonymized context only.  
All legal materials, client information, and case data remain **entirely local and encrypted**.

```

+-----------------------+
|  User (Lawyer)        |
|  ─ Query or Upload ─▶ |
+-----------┬-----------+
|
v
+-----------------------+
| Local MCP Node        |
|  • Ingest legal docs  |
|  • Sanitize & embed   |
|  • Retrieve context   |
|  • Call GPT-5 safely  |
+-----------┬-----------+
|
v
+-----------------------+
| GPT-5 Reasoning API   |
|  • No memory          |
|  • Receives only      |
|    redacted context   |
+-----------┬-----------+
|
v
+-----------------------+
| MCP Post-Processor    |
|  • Re-inject private  |
|    names, refs        |
|  • Store logs locally |
+-----------------------+

````

---

## 2. Core Components

### 🧩 `mcp-gateway`  
**Language:** Rust / Node.js  
**Purpose:**  
- Acts as firewall and broker between Counsel AI and GPT-5.  
- Implements the **Model Context Protocol (MCP)** for structured message passing.  
- Handles context formatting and zero-leakage serialization.

**Endpoints:**
| Method | Route | Description |
|---------|--------|-------------|
| `POST /query` | Accepts user query + file refs |
| `GET /context/:id` | Returns anonymized legal context |
| `POST /reason` | Sends prompt bundle to GPT-5 |
| `POST /verify` | Validates GPT response integrity |
| `POST /store` | Commits local log / audit entry |

---

### 🧠 `gpt5-reasoner`
**Type:** External API (OpenAI GPT-5)  
**Role:**  
- Pure reasoning engine, no state retention.  
- Receives sanitized legal contexts (no names, identifiers).  
- Returns structured legal logic in JSON or Markdown.

**Example Input:**
```json
{
  "prompt": "Under Italian Civil Code Art. 2043, summarize liability for unlawful acts.",
  "context": [
    "Art.2043 CC – Risarcimento per fatto illecito",
    "Cass. Civ. Sez. III n. 1234/2021 — Principio di colpa"
  ]
}
````

**Example Output:**

```json
{
  "summary": "Liability arises from any intentional or negligent act causing unjust damage...",
  "citations": ["Art. 2043 CC", "Cass. Civ. Sez. III n. 1234/2021"]
}
```

---

### 📚 `legal-kb`

**Backend:** Local SQLite / DuckDB + Vector DB (LanceDB / Qdrant)
**Functions:**

* Stores canonical sources (Codice Civile, Giurisprudenza Italiana, Dottrina).
* Creates embeddings locally via open-source model (`text-embedding-3-large` offline copy).
* Supports semantic search and retrieval without exposing data externally.

---

### 🔐 `security-core`

**Responsibilities:**

* AES-256-GCM encryption for all storage.
* libsodium signatures for integrity.
* Secure enclave for private keys.
* Optional hardware token or YubiKey integration.

**Key Directories:**

```
/data/encrypted/clients.db
/data/encrypted/opinions/
/logs/audit.jsonl
```

---

### 💬 `ui-desktop`

**Framework:** React + Tauri (cross-platform)
**Features:**

* Local-only interface.
* File drop for case materials (PDF, DOCX).
* Chat-style reasoning dialogue.
* “Generate Legal Opinion” button → triggers `/query` endpoint.
* Local Markdown viewer for GPT reasoning results.

---

## 3. Data Flow (Privacy-Safe)

| Step | Action                               | Data Visibility    |
| ---- | ------------------------------------ | ------------------ |
| 1    | User uploads case file               | Local only         |
| 2    | MCP parses + chunks                  | Local              |
| 3    | Context vectorized                   | Local              |
| 4    | MCP sends sanitized context to GPT-5 | Abstract text only |
| 5    | GPT-5 returns reasoning              | Abstract text      |
| 6    | MCP reinjects original refs          | Local only         |
| 7    | Opinion saved / exported             | Encrypted file     |

---

## 4. Privacy & Compliance

| Measure                      | Description                                                     |
| ---------------------------- | --------------------------------------------------------------- |
| **Zero Cloud Storage**       | No documents leave the device/server                            |
| **No Persistent LLM Memory** | GPT-5 stateless calls only                                      |
| **Local Logs Only**          | Encrypted JSON logs with timestamp and hash                     |
| **GDPR Alignment**           | Data subject rights preserved                                   |
| **Auditability**             | Every reasoning call is timestamped via `HashTimer`-style proof |

---

## 5. Integration Options

| Target                       | Description                                                   |
| ---------------------------- | ------------------------------------------------------------- |
| **Legal Database APIs**      | Integrate with Italgiure, DeJure, or Altalex feeds            |
| **Court Management Systems** | Export structured opinions via XML                            |
| **Counsel AI Network**       | Optional P2P MCP link to share anonymized reasoning templates |

---

## 6. Future Extensions

* Offline local LLM for emergency reasoning (Mini-GPT or Mistral 7B instruct).
* zk-Proof certification of opinions (`zk-STARK signature`) for timestamped authenticity.
* Cross-jurisdiction reasoning plug-ins (EU Law, ECHR, GDPR modules).
* Voice-based interface using local Whisper model.
* Secure containerized deployment (`Dockerfile.mcp`, `docker-compose.local.yml`).

---

## 7. Deployment Example

```bash
# 1. Clone repository
git clone https://github.com/dmrl789/CounselAI.git
cd CounselAI

# 2. Start local MCP node
docker compose up -d mcp-gateway

# 3. Launch desktop UI
cd apps/ui-desktop && npm run tauri dev

# 4. Open localhost:5142
# Begin private legal reasoning session
```

---

## 8. Security Summary

| Risk                         | Mitigation                             |
| ---------------------------- | -------------------------------------- |
| LLM exposure of private data | Context sanitization + redaction rules |
| Unauthorized access          | Encrypted storage + token auth         |
| Data persistence             | Local-only, periodic purge             |
| Model drift or bias          | Jurisprudence bias monitor in MCP      |
| Legal citation errors        | Cross-check with `legal-kb` database   |

---

## 9. License & Ownership

* **All code and datasets** remain property of **Counsel AI / IPPAN Labs**.
* GPT-5 is used *solely* as a reasoning engine; no client or case data are transmitted.

---

### ✅ Summary

> Counsel AI uses a **local, encrypted MCP** to connect safely to GPT-5 for legal reasoning.
> It guarantees that **no personal, confidential, or case data ever leave your device** while still enabling powerful legal analysis and opinion generation.

---

**End of Document**

```

---

Would you like me to add a **companion file** (`docker-compose.local.yml`) that launches:
- the MCP gateway,
- a local Qdrant vector DB,
- and the React + Tauri desktop app  

so you can run this architecture on your own server or laptop immediately?
```
