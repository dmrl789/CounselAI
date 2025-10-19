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

## Notes
- Offline corpus should be placed in `counsel_ai/corpus/` (not tracked).
- This MVP avoids network calls and external LLM usage.
- See PRD for architecture and roadmap.
