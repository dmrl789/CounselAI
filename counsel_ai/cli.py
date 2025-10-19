from __future__ import annotations
import json
from pathlib import Path
import typer
from rich.console import Console
from .dialogue import interactive_intake
from .reasoning import build_reasoning, draft_opinion
from .docgen import generate_documents
from .audit import append_record

app = typer.Typer(add_completion=False)
console = Console()


@app.command()
def intake(case_id: str = typer.Option(None, "--case-id", help="ID pratica")):
    case = interactive_intake(case_id)
    console.print_json(data=case.model_dump())


@app.command()
def opinion(
    case_path: Path = typer.Argument(..., help="Path a CaseFile.json"),
    output_json: Path = typer.Option(None, "--out", help="File JSON di output"),
):
    case = json.loads(case_path.read_text(encoding="utf-8"))
    from .models import CaseFile  # local import to avoid cli import weight

    case_obj = CaseFile.model_validate(case)
    reasoning = build_reasoning(case_obj)
    opin = draft_opinion(case_obj, reasoning)
    append_record("opinion_generated", {
        "case_id": case_obj.case_id,
        "citations": opin.citations,
        "summary": opin.summary,
    })

    if output_json:
        output_json.write_text(json.dumps(opin.model_dump(), ensure_ascii=False, indent=2), encoding="utf-8")
    console.print("[green]Parere generato.[/green]")
    console.print_json(data=opin.model_dump())


@app.command()
def export(
    case_path: Path = typer.Argument(..., help="Path a CaseFile.json"),
    opinion_path: Path = typer.Argument(..., help="Path a Opinion.json"),
):
    case = json.loads(case_path.read_text(encoding="utf-8"))
    opin = json.loads(opinion_path.read_text(encoding="utf-8"))
    from .models import CaseFile, Opinion

    case_obj = CaseFile.model_validate(case)
    opinion_obj = Opinion.model_validate(opin)

    files = generate_documents(case_obj, opinion_obj)
    append_record("documents_exported", {"case_id": case_obj.case_id, **files})
    console.print(f"[green]Esportati:[/green] {files}")


if __name__ == "__main__":
    app()
