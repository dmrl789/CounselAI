# ⚖️ Counsel AI — Product Requirements Document (PRD)
**Version:** v1.0  
**Date:** 19 Oct 2025  
**Author:** IPPAN Labs / Désirée Verga  
**Confidential** — internal document, not for public redistribution  

---

## 1. Vision

> “An Italian lawyer in silicon form.”  

Counsel AI is an intelligent, privacy-preserving legal counsel that understands **Italian law and jurisprudence**, reasons with precedent and doctrine, and drafts compliant legal acts or opinions after interactive consultation — **without leaking private data**.  
It operates through its own **protected Model Context Protocol (MCP)** enclave, ensuring full confidentiality and verifiable reasoning integrity.

---

## 2. Objectives

| Objective | Description |
|------------|-------------|
| **Legal Accuracy** | Deliver opinions aligned with Italian civil, penal, and administrative law using case law and doctrine. |
| **Confidentiality** | Ensure zero data leakage through sandboxed MCP runtime, encrypted local memory, and offline reasoning. |
| **Explainability** | Provide structured reasoning chains citing Codice Civile, Cassazione sentences, and doctrinal sources. |
| **Automation** | Draft acts (ricorsi, contratti, diffide) after gathering case facts through chat. |
| **Compliance** | Adhere to GDPR, professional-secrecy standards, and EU AI Act regulations. |

---

## 3. Core Use Cases

| ID | Title | Description |
|----|--------|-------------|
| UC-1 | Legal Opinion Assistant | User presents a case; Counsel AI outputs structured *parere legale* with citations. |
| UC-2 | Act Drafting | After dialogue, generate a formatted Italian legal act ready for filing. |
| UC-3 | Doctrine Lookup | Retrieve authoritative commentary (Giurisprudenza, Dottrina, Cassazione). |
| UC-4 | Client Consultation | Simulate Q&A with a client to collect legally relevant facts. |
| UC-5 | Risk Assessment | Identify legal exposure, norms, limitation periods, procedural steps. |

---

## 4. Functional Requirements

### 4.1 Legal Reasoning Engine
- LLM fine-tuned on Italian legal corpus.  
- Deterministic reasoning with citations and timestamps.  
- Temporal logic: law validity by date.  
- Explainability via structured JSON tree.

### 4.2 Secure MCP Runtime
- Air-gapped MCP; no external calls.  
- AES-256 encrypted vault.  
- Zero-knowledge proof of computation (optional).  
- Role-based access.

### 4.3 Dialogue & Case Builder
- Chat-driven structured intake (facts, parties, timeline).  
- Generates `CaseFile.json`.

### 4.4 Document Generator
- Templates for major act types.  
- Auto-fill from case facts.  
- Outputs: DOCX, PDF, RTF.  
- Digital-signature stub integration.

### 4.5 Knowledge Base
- Offline legal corpus: Codici, Cassazione, Dottrina.  
- Searchable via local index.

### 4.6 Audit & Explainability
- Reasoning tree generation.  
- Exportable logs (JSON/PDF).

---

## 5. Non-Functional Requirements

| Category | Requirement |
|-----------|-------------|
| **Security** | AES-256 encryption, TLS 1.3, zk-proof option. |
| **Privacy** | Local-only processing. |
| **Latency** | Opinion generation < 5 s. |
| **Scalability** | Multi-tenant via containers. |
| **Compliance** | GDPR, CNF, eIDAS II ready. |

---

## 6. Architecture Overview

```

+--------------------------------------------------------------+

| Counsel AI MCP Node                                              |
| ---------------------------------------------------------------- |
| 1. Secure Chat Gateway (Tauri / React)                           |
| 2. Dialogue Orchestrator (fact intake)                           |
| 3. Legal Reasoning Core (LLM + symbolic reasoning)               |
| 4. Doctrine Index (SQLite + vector store)                        |
| 5. Document Generator (Template Engine)                          |
| 6. Audit & Logging (immutable local ledger)                      |
| 7. Encryption Layer (AES-256)                                    |
| +--------------------------------------------------------------+ |

````

---

## 7. Workflow

1. Secure session starts in local MCP.  
2. Dialogue gathers facts.  
3. Reasoning engine analyzes norms + cases.  
4. Opinion generated with citations.  
5. Act generated and signed.  
6. Audit ledger updated.

---

## 8. Data Model

```json
{
  "case_id": "HT-2025-00123",
  "client": {"name": "Mario Rossi", "role": "Ricorrente"},
  "facts": ["Contract breach", "Payment missing"],
  "jurisdiction": "Tribunale di Milano",
  "applicable_law": ["art. 1218 c.c.", "Cass. Civ. 30574/2022"],
  "recommendation": "Ricorso per inadempimento",
  "documents": ["ricorso.docx", "parere.pdf"]
}
````

---

## 9. Technical Stack

| Layer        | Technology                 |
| ------------ | -------------------------- |
| Core Runtime | Rust + Tauri               |
| LLM          | GPT-5 fine-tuned (offline) |
| DB           | SQLite + Qdrant            |
| Crypto       | RustCrypto / libsodium     |
| UI           | React + Tailwind           |
| Output       | python-docx + reportlab    |

---

## 10. Regulatory & Ethical Alignment

* GDPR-compliant local processing.
* Forensic secrecy per art. 622 CP.
* Explainable AI (EU AI Act).
* Human-in-loop review.
* Disclaimers on all opinions.

---

## 11. Future Extensions

* Integration with Notartel / Giustizia Digitale.
* Cross-border EU law support.
* Voice dictation.
* Blockchain anchoring.

---

## 12. Success Metrics

| KPI                 | Target |
| ------------------- | ------ |
| Opinion time        | ≤ 5 s  |
| Legal accuracy      | ≥ 95 % |
| Satisfaction        | ≥ 90 % |
| Data leaks          | 0      |
| Formatting validity | ≥ 99 % |

---

## 13. Risk Assessment

| Risk                | Mitigation         |
| ------------------- | ------------------ |
| Misinterpretation   | Human validation   |
| Data leakage        | Air-gapped MCP     |
| Obsolete law        | Corpus auto-update |
| Unauthorized access | Role-based keys    |

---

## 14. Launch Plan (MVP → v1)

| Phase | Deliverable          | Description            |
| ----- | -------------------- | ---------------------- |
| P1    | MVP Chat + Opinion   | Secure local reasoning |
| P2    | Legal Acts Generator | DOCX/PDF output        |
| P3    | Doctrine Corpus      | Jurisprudence DB       |
| P4    | Secure MCP           | Hardened enclave       |
| P5    | Law-Firm Integration | Multi-user + audit     |

---

## 15. Suggested Next Steps

1. Architect Rust MCP sandbox.
2. Prepare legal corpus ingestion.
3. Define dialogue schema.
4. Build template library.
5. Integrate local LLM inference.

---

## 16. Security & Compliance Specification

### 16.1 Overview

Complies with GDPR, eIDAS II, AI Act, CNF ethics, art. 622 CP.
Privacy-by-design and forensic secrecy enforced.

### 16.2 Security Objectives

| Objective       | Description                       |
| --------------- | --------------------------------- |
| Confidentiality | AES-256 vaults, no outbound data  |
| Integrity       | HashTimer ledger or FinDAG anchor |
| Accountability  | Signed audit records              |
| Non-repudiation | Ed25519 document signatures       |

### 16.3 Data Handling Policy

| Data Type | Retention     | Storage          |
| --------- | ------------- | ---------------- |
| Chats     | ≤ 30 days     | Encrypted vault  |
| Opinions  | Indefinite    | Local            |
| Docs      | As per policy | Local            |
| Logs      | ≥ 3 years     | HashTimer ledger |

> No external APIs, PGP-signed updates only.

### 16.4 Roles

| Role    | Auth              |
| ------- | ----------------- |
| Lawyer  | FIDO2 / CNS       |
| Client  | OTP               |
| Auditor | Read-only keypair |

### 16.5 Governance

* Model card + dataset fingerprint.
* Annual bias & accuracy review.
* Dual signature for model updates.

### 16.6 Safeguards

* AI advisory disclaimer.
* Expandable reasoning tree.
* Right-to-erasure confirmed cryptographically.

### 16.7 Technical Controls

AES-256-GCM, libsodium, sandboxed runtime, PGP-signed updates.

### 16.8 Certifications

Target: ISO 27001, 27701, ETSI EN 319 401, CNF compliance, AI Act conformity.

### 16.9 Incident Response

Lockdown → Notify → Audit → Forensics → Rebuild.

---

## 17. Implementation Roadmap & Milestones

| Phase | Objective           | Duration |
| ----- | ------------------- | -------- |
| P1    | Secure MCP Core     | 6 weeks  |
| P2    | Legal Reasoning MVP | 8 weeks  |
| P3    | Document Generator  | 6 weeks  |
| P4    | Doctrine Expansion  | 8 weeks  |
| P5    | Enterprise & Audit  | 10 weeks |

### Module Ownership

| Module                | Language    | Repo                  |
| --------------------- | ----------- | --------------------- |
| MCP Runtime           | Rust        | `counsel-ai/mcp`      |
| Dialogue Orchestrator | Python      | `counsel-ai/core`     |
| Doctrine Indexer      | Python      | `counsel-ai/doctrine` |
| Document Generator    | Python      | `counsel-ai/docgen`   |
| UI                    | React/Tauri | `counsel-ai/ui`       |
| Audit Ledger          | Rust        | `counsel-ai/audit`    |

### QA Plan

* Legal accuracy benchmarks.
* GDPR audits.
* Security pentests.
* CI regression tests.

---

## 18. Commercial Model, Licensing & Market Positioning

### 18.1 Vision

First **privacy-certified AI lawyer** for Italian and EU markets.

### 18.2 Target Segments

Law firms, corporates, notaries, PA, academia.

### 18.3 Market Positioning

| Competitor     | Position               |
| -------------- | ---------------------- |
| ChatGPT        | Cloud, general         |
| Lexis+ AI      | Accurate, US-centric   |
| **Counsel AI** | 🇮🇹 Private + Precise |

**Tagline:** “AI that practices Italian law — securely, privately, verifiably.”

### 18.4 Deployment Models

On-prem MCP, Private Cloud MCP, Hybrid Edge, SaaS Sandbox.

### 18.5 Licensing & Pricing

| Tier           | Users | Monthly   | Features           |
| -------------- | ----- | --------- | ------------------ |
| Solo Lex       | 1     | € 79      | Basic reasoning    |
| Studio Pro     | ≤ 10  | € 499     | Full corpus, audit |
| Enterprise Lex | 10 +  | € 1 200 + | Multi-tenant, API  |
| Public Edition | —     | Custom    | PA / Education     |
| OEM            | —     | Royalty   | SDK integration    |

### 18.6 Revenue Streams

Subscriptions, customization, training, content licensing, audit anchoring.

### 18.7 Value Proposition

Confidentiality + Explainability + Compliance + Speed.

### 18.8 Partnerships

CNF, Universities, ArubaCloud, FinDAG, eIDAS providers.

### 18.9 Go-To-Market

| Stage           | Period  | Outcome    |
| --------------- | ------- | ---------- |
| MVP             | Q1 2026 | Pilot      |
| Partner Firms   | Q2 2026 | Validation |
| Public Beta     | Q3 2026 | Adoption   |
| Enterprise v2.0 | Q4 2026 | Revenue    |
| EU Expansion    | 2027    | Growth     |

### 18.10 Branding

Logo ⚖️ Counsel AI — “Lex in Silicio”
Formal, authoritative, ethical tone.

### 18.11 Financial Model

| Metric       | Y1     | Y2    | Y3    |
| ------------ | ------ | ----- | ----- |
| Paying Nodes | 150    | 500   | 1 200 |
| ARR (€)      | 0.72 M | 2.7 M | 6.9 M |
| Margin       | 82 %   | 86 %  | 88 %  |

### 18.12 Long-Term Strategy

Integration with publishers and Giustizia Digitale; EU rollout; Counsel AI Foundation.

### 18.13 Success Metrics

| KPI                       | Target         |
| ------------------------- | -------------- |
| Market share (Italy)      | ≥ 25 % by 2027 |
| Renewal rate              | ≥ 90 %         |
| NPS                       | ≥ 9/10         |
| Accuracy                  | ≥ 95 %         |
| Compliance incidents      | 0              |
| ISO/AI Act certifications | ≥ 3 by 2027    |

---

## ✅ Document Status

**Internal PRD — ready for GitHub upload**
Path suggestion: `docs/PRD.md`
Commit message:

> `docs: add Counsel AI v1.0 Product Requirements Document`

````

---

### ✅ Upload Instructions

1. In your local repo folder:
   ```bash
   cd C:\Users\yuyby\IPPAN_LABS\CounselAI
   mkdir -p docs
   code docs/PRD.md
````

2. Paste the full Markdown file above.
3. Save, then commit and push:

   ```bash
   git add docs/PRD.md
   git commit -m "docs: add Counsel AI v1.0 PRD"
   git push origin main
   ```

Would you like me to prepare next a **GitHub repository structure** (`/mcp`, `/ui`, `/doctrine`, `/docgen`, `/audit`, `/docs`) with initial README templates for each folder, matching this PRD?
