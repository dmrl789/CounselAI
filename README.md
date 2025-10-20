# Counsel AI (MVP)

Local, privacy-preserving Italian legal assistant MVP per the internal PRD in `docs/PRD.md`.

## Features (MVP)
- Chat-style CLI intake to build a `CaseFile.json`
- Simple reasoning producing an Opinion with basic citations
- Document generation to DOCX and PDF using a Jinja2 template
- Hash-chained local audit ledger

## Install

```bash
python -m venv .venv && source .venv/bin/activate
pip install -e .
```

## Usage

1) Intake and save case file:

```bash
counsel-ai intake --case-id HT-2025-0001 > CaseFile.json
```

2) Generate opinion JSON:

```bash
counsel-ai opinion CaseFile.json --out Opinion.json
```

3) Export documents (DOCX, PDF) to `output/`:

```bash
counsel-ai export CaseFile.json Opinion.json
```

Generated files are stored under `output/`. Audit entries are appended to `audit/ledger.jsonl`.

## Desktop MCP Gateway & Offline Reasoning

The `services/mcp-gateway` crate exposes REST endpoints used by Counsel AI Desktop. It
prefers OpenAI for online reasoning when `OPENAI_API_KEY` is configured, and
automatically falls back to a local [`llama.cpp`](https://github.com/ggerganov/llama.cpp)
model when offline. Configure the gateway via environment variables (see
`services/mcp-gateway/.env.example`).

To run the gateway locally:

```bash
cd services/mcp-gateway
cargo run --release
```

Download a quantized Mistral 7B model into `services/mcp-gateway/models/`:

```bash
mkdir -p services/mcp-gateway/models
cd services/mcp-gateway/models
wget https://huggingface.co/TheBloke/Mistral-7B-Instruct-v0.2-GGUF/resolve/main/mistral-7b-instruct-v0.2.Q4_K_M.gguf
```

The Dockerfile in `services/mcp-gateway/` packages the gateway together with the
local model for offline deployments.

## Notes
- Offline corpus should be placed in `counsel_ai/corpus/` (not tracked).
- This MVP avoids network calls and external LLM usage.
- See PRD for architecture and roadmap.
