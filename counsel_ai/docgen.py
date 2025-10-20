from __future__ import annotations
import logging
from pathlib import Path
from typing import Dict
from jinja2 import Environment, FileSystemLoader, select_autoescape, TemplateNotFound
from docx import Document
from reportlab.pdfgen import canvas
from reportlab.lib.pagesizes import A4
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from .models import CaseFile, Opinion

logger = logging.getLogger(__name__)

TEMPLATES_DIR = Path(__file__).resolve().parent / "templates"
OUTPUT_DIR = Path.cwd() / "output"


def _ensure_dirs() -> None:
    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)


def render_parere_text(case: CaseFile, opinion: Opinion) -> str:
    """Render parere text from template with error handling"""
    try:
        logger.info(f"Rendering parere text for case {case.case_id}")

        if not TEMPLATES_DIR.exists():
            raise FileNotFoundError(f"Templates directory not found: {TEMPLATES_DIR}")

        env = Environment(
            loader=FileSystemLoader(str(TEMPLATES_DIR)),
            autoescape=select_autoescape()
        )

        try:
            template = env.get_template("parere.md.j2")
        except TemplateNotFound:
            logger.error("Template parere.md.j2 not found")
            raise FileNotFoundError("Template parere.md.j2 not found")

        rendered = template.render(case=case.model_dump(), opinion=opinion.model_dump())
        logger.info("Successfully rendered parere text")
        return rendered

    except Exception as e:
        logger.error(f"Error rendering parere text: {e}", exc_info=True)
        # Return a basic fallback template
        return f"""# {opinion.title}

Generato il: {opinion.generated_at}

## Sommario
{opinion.summary}

## Raccomandazioni
{chr(10).join(f"- {rec}" for rec in opinion.recommendations)}

## Citazioni
{chr(10).join(f"- {cit}" for cit in opinion.citations)}

## Dati di caso
- ID: {case.case_id}
- Cliente: {case.client.name} ({case.client.role})
- Giurisdizione: {case.jurisdiction or "N/D"}
- Fatti:
{chr(10).join(f"- {f}" for f in case.facts)}
"""


def write_docx(text: str, path: Path) -> None:
    """Write DOCX document with error handling"""
    try:
        logger.info(f"Writing DOCX to {path}")

        doc = Document()

        # Split text into paragraphs and add to document
        paragraphs = text.split("\n\n")
        for paragraph in paragraphs:
            if paragraph.strip():
                doc.add_paragraph(paragraph.strip())

        # Ensure directory exists
        path.parent.mkdir(parents=True, exist_ok=True)

        doc.save(str(path))
        logger.info(f"Successfully wrote DOCX to {path}")

    except Exception as e:
        logger.error(f"Error writing DOCX to {path}: {e}", exc_info=True)
        raise


def write_pdf(text: str, path: Path) -> None:
    """Write PDF document with error handling and better formatting"""
    try:
        logger.info(f"Writing PDF to {path}")

        # Ensure directory exists
        path.parent.mkdir(parents=True, exist_ok=True)

        c = canvas.Canvas(str(path), pagesize=A4)
        width, height = A4
        x, y = 40, height - 40
        line_height = 14
        margin_bottom = 60

        lines = text.splitlines()
        for line in lines:
            if not line.strip():
                y -= line_height
                continue

            # Handle long lines by wrapping
            if len(line) > 110:
                words = line.split()
                current_line = ""
                for word in words:
                    if len(current_line + " " + word) <= 110:
                        current_line += (" " + word) if current_line else word
                    else:
                        if current_line:
                            c.drawString(x, y, current_line)
                            y -= line_height
                        current_line = word
                if current_line:
                    c.drawString(x, y, current_line)
            else:
                c.drawString(x, y, line)

            y -= line_height

            # Check if we need a new page
            if y < margin_bottom:
                c.showPage()
                y = height - 40

        c.save()
        logger.info(f"Successfully wrote PDF to {path}")

    except Exception as e:
        logger.error(f"Error writing PDF to {path}: {e}", exc_info=True)
        raise


def generate_documents(case: CaseFile, opinion: Opinion) -> Dict[str, str]:
    """Generate DOCX and PDF documents with comprehensive error handling"""
    try:
        logger.info(f"Generating documents for case {case.case_id}")

        _ensure_dirs()
        text = render_parere_text(case, opinion)

        # Sanitize case_id for filename
        safe_case_id = "".join(c for c in case.case_id if c.isalnum() or c in "._-")
        docx_path = OUTPUT_DIR / f"parere_{safe_case_id}.docx"
        pdf_path = OUTPUT_DIR / f"parere_{safe_case_id}.pdf"

        # Generate documents
        write_docx(text, docx_path)
        write_pdf(text, pdf_path)

        result = {"docx": str(docx_path), "pdf": str(pdf_path)}
        logger.info(f"Successfully generated documents: {result}")
        return result

    except Exception as e:
        logger.error(
            f"Error generating documents for case {
                case.case_id}: {e}",
            exc_info=True)
        raise
