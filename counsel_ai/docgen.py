from __future__ import annotations
from pathlib import Path
from typing import Dict
from jinja2 import Environment, FileSystemLoader, select_autoescape
from docx import Document
from reportlab.pdfgen import canvas
from reportlab.lib.pagesizes import A4
from .models import CaseFile, Opinion

TEMPLATES_DIR = Path(__file__).resolve().parent / "templates"
OUTPUT_DIR = Path.cwd() / "output"


def _ensure_dirs() -> None:
    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)


def render_parere_text(case: CaseFile, opinion: Opinion) -> str:
    env = Environment(
        loader=FileSystemLoader(str(TEMPLATES_DIR)),
        autoescape=select_autoescape()
    )
    template = env.get_template("parere.md.j2")
    return template.render(case=case.model_dump(), opinion=opinion.model_dump())


def write_docx(text: str, path: Path) -> None:
    doc = Document()
    for paragraph in text.split("\n\n"):
        doc.add_paragraph(paragraph)
    doc.save(path)


def write_pdf(text: str, path: Path) -> None:
    c = canvas.Canvas(str(path), pagesize=A4)
    width, height = A4
    x, y = 40, height - 40
    for line in text.splitlines():
        if not line.strip():
            y -= 16
            continue
        c.drawString(x, y, line[:110])
        y -= 14
        if y < 60:
            c.showPage()
            y = height - 40
    c.save()


def generate_documents(case: CaseFile, opinion: Opinion) -> Dict[str, str]:
    _ensure_dirs()
    text = render_parere_text(case, opinion)
    docx_path = OUTPUT_DIR / f"parere_{case.case_id}.docx"
    pdf_path = OUTPUT_DIR / f"parere_{case.case_id}.pdf"
    write_docx(text, docx_path)
    write_pdf(text, pdf_path)
    return {"docx": str(docx_path), "pdf": str(pdf_path)}
