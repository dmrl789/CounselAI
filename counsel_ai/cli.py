from __future__ import annotations

import json
import logging
import sys
from pathlib import Path
from typing import Optional

import typer
from rich.console import Console
from rich.logging import RichHandler

from .audit import append_record
from .dialogue import interactive_intake
from .docgen import generate_documents
from .reasoning import build_reasoning, draft_opinion

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s - %(name)s - %(levelname)s - %(message)s",
    handlers=[RichHandler(rich_tracebacks=True)],
)
logger = logging.getLogger(__name__)

app = typer.Typer(add_completion=False)
console = Console()


@app.command()
def intake(
    case_id: Optional[str] = typer.Option(None, "--case-id", help="ID pratica")
) -> None:
    """Interactive case intake to build a CaseFile.json"""
    try:
        logger.info(f"Starting case intake for case_id: {case_id}")
        case = interactive_intake(case_id)
        logger.info(f"Successfully created case file for {case.case_id}")
        console.print_json(data=case.model_dump())
    except Exception as e:
        logger.error(f"Failed to complete case intake: {e}", exc_info=True)
        console.print(f"[red]Error during case intake: {e}[/red]")
        sys.exit(1)


@app.command()
def opinion(
    case_path: Path = typer.Argument(..., help="Path a CaseFile.json"),
    output_json: Optional[Path] = typer.Option(
        None, "--out", help="File JSON di output"
    ),
) -> None:
    """Generate legal opinion from case file"""
    try:
        logger.info(f"Generating opinion for case file: {case_path}")

        if not case_path.exists():
            raise FileNotFoundError(f"Case file not found: {case_path}")

        case_data = json.loads(case_path.read_text(encoding="utf-8"))
        from .models import CaseFile  # local import to avoid cli import weight

        case_obj = CaseFile.model_validate(case_data)
        logger.info(f"Validated case file for case_id: {case_obj.case_id}")

        reasoning = build_reasoning(case_obj)
        logger.info("Built reasoning tree")

        opin = draft_opinion(case_obj, reasoning)
        logger.info("Drafted opinion")

        append_record(
            "opinion_generated",
            {
                "case_id": case_obj.case_id,
                "citations": opin.citations,
                "summary": opin.summary,
            },
        )
        logger.info("Recorded opinion generation in audit ledger")

        if output_json:
            output_json.write_text(
                json.dumps(opin.model_dump(), ensure_ascii=False, indent=2),
                encoding="utf-8",
            )
            logger.info(f"Opinion saved to: {output_json}")

        console.print("[green]Parere generato.[/green]")
        console.print_json(data=opin.model_dump())

    except FileNotFoundError as e:
        logger.error(f"File not found: {e}")
        console.print(f"[red]File not found: {e}[/red]")
        sys.exit(1)
    except json.JSONDecodeError as e:
        logger.error(f"Invalid JSON in case file: {e}")
        console.print(f"[red]Invalid JSON in case file: {e}[/red]")
        sys.exit(1)
    except Exception as e:
        logger.error(f"Failed to generate opinion: {e}", exc_info=True)
        console.print(f"[red]Error generating opinion: {e}[/red]")
        sys.exit(1)


@app.command()
def export(
    case_path: Path = typer.Argument(..., help="Path a CaseFile.json"),
    opinion_path: Path = typer.Argument(..., help="Path a Opinion.json"),
) -> None:
    """Export case and opinion to DOCX and PDF documents"""
    try:
        logger.info(
            f"Exporting documents for case: {case_path}, opinion: {opinion_path}"
        )

        if not case_path.exists():
            raise FileNotFoundError(f"Case file not found: {case_path}")
        if not opinion_path.exists():
            raise FileNotFoundError(f"Opinion file not found: {opinion_path}")

        case_data = json.loads(case_path.read_text(encoding="utf-8"))
        opinion_data = json.loads(opinion_path.read_text(encoding="utf-8"))
        from .models import CaseFile, Opinion

        case_obj = CaseFile.model_validate(case_data)
        opinion_obj = Opinion.model_validate(opinion_data)
        logger.info(f"Validated case and opinion files for case_id: {case_obj.case_id}")

        files = generate_documents(case_obj, opinion_obj)
        logger.info(f"Generated documents: {files}")

        append_record("documents_exported", {"case_id": case_obj.case_id, **files})
        logger.info("Recorded document export in audit ledger")

        console.print(f"[green]Esportati:[/green] {files}")

    except FileNotFoundError as e:
        logger.error(f"File not found: {e}")
        console.print(f"[red]File not found: {e}[/red]")
        sys.exit(1)
    except json.JSONDecodeError as e:
        logger.error(f"Invalid JSON in file: {e}")
        console.print(f"[red]Invalid JSON in file: {e}[/red]")
        sys.exit(1)
    except Exception as e:
        logger.error(f"Failed to export documents: {e}", exc_info=True)
        console.print(f"[red]Error exporting documents: {e}[/red]")
        sys.exit(1)


if __name__ == "__main__":
    app()
