from __future__ import annotations
from typing import Optional
from pydantic import ValidationError
from rich.prompt import Prompt, Confirm
from rich.console import Console
from .models import Party, CaseFile

console = Console()


def _ask_non_empty(prompt: str) -> str:
    while True:
        value = Prompt.ask(prompt).strip()
        if value:
            return value
        console.print("[red]Campo obbligatorio, riprova.[/red]")


def interactive_intake(existing_case_id: Optional[str] = None) -> CaseFile:
    console.print("[bold]Raccolta dati del caso (MVP)[/bold]")
    case_id = existing_case_id or _ask_non_empty("ID pratica (es. HT-2025-0001)")
    client_name = _ask_non_empty("Nome cliente")
    client_role = Prompt.ask(
        "Ruolo cliente",
        choices=[
            "Ricorrente",
            "Resistente",
            "Attore",
            "Convenuto",
            "Cliente",
            "Controparte",
        ],
        default="Cliente",
        show_choices=False,
    )

    facts: list[str] = []
    console.print("Inserisci fatti rilevanti (lascia vuoto per terminare)")
    while True:
        fact = Prompt.ask("Fatto")
        if not fact.strip():
            break
        facts.append(fact.strip())

    applicable_law: list[str] = []
    console.print("Norme/casi applicabili (es. 'art. 1218 c.c.', 'Cass. Civ. 30574/2022')")
    while True:
        norm = Prompt.ask("Norma/Caso")
        if not norm.strip():
            break
        applicable_law.append(norm.strip())

    jurisdiction = Prompt.ask("Giurisdizione (es. Tribunale di Milano)", default="") or None

    try:
        case_file = CaseFile(
            case_id=case_id,
            client=Party(name=client_name, role=client_role),
            parties=[Party(name=client_name, role=client_role)],
            facts=facts,
            jurisdiction=jurisdiction,
            applicable_law=applicable_law,
        )
        return case_file
    except ValidationError as exc:
        console.print(f"[red]Errore di validazione: {exc}[/red]")
        raise
