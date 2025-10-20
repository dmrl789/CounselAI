from __future__ import annotations
import logging
from typing import Optional
from pydantic import ValidationError
from rich.prompt import Prompt, Confirm
from rich.console import Console
from .models import Party, CaseFile

console = Console()
logger = logging.getLogger(__name__)


def _ask_non_empty(prompt: str, max_attempts: int = 3) -> str:
    """Ask for non-empty input with retry limit"""
    attempts = 0
    while attempts < max_attempts:
        try:
            value = Prompt.ask(prompt).strip()
            if value:
                return value
            attempts += 1
            if attempts < max_attempts:
                console.print(
                    f"[red]Campo obbligatorio, riprova ({attempts}/{max_attempts}).[/red]")
            else:
                console.print("[red]Troppi tentativi falliti.[/red]")
                raise ValueError("Maximum attempts exceeded for required field")
        except KeyboardInterrupt:
            console.print("\n[yellow]Operazione annullata dall'utente.[/yellow]")
            raise
        except Exception as e:
            logger.error(f"Error in _ask_non_empty: {e}")
            raise


def interactive_intake(existing_case_id: Optional[str] = None) -> CaseFile:
    """Interactive case intake with comprehensive error handling"""
    try:
        logger.info("Starting interactive case intake")
        console.print("[bold]Raccolta dati del caso (MVP)[/bold]")

        case_id = existing_case_id or _ask_non_empty("ID pratica (es. HT-2025-0001)")
        logger.info(f"Case ID: {case_id}")

        client_name = _ask_non_empty("Nome cliente")
        logger.info(f"Client name: {client_name}")

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
        logger.info(f"Client role: {client_role}")

        facts: list[str] = []
        console.print("Inserisci fatti rilevanti (lascia vuoto per terminare)")
        fact_count = 0
        while fact_count < 50:  # Reasonable limit
            try:
                fact = Prompt.ask("Fatto")
                if not fact.strip():
                    break
                facts.append(fact.strip())
                fact_count += 1
            except KeyboardInterrupt:
                console.print("\n[yellow]Interruzione inserimento fatti.[/yellow]")
                break

        logger.info(f"Collected {len(facts)} facts")

        applicable_law: list[str] = []
        console.print(
            "Norme/casi applicabili (es. 'art. 1218 c.c.', 'Cass. Civ. 30574/2022')")
        law_count = 0
        while law_count < 20:  # Reasonable limit
            try:
                norm = Prompt.ask("Norma/Caso")
                if not norm.strip():
                    break
                applicable_law.append(norm.strip())
                law_count += 1
            except KeyboardInterrupt:
                console.print("\n[yellow]Interruzione inserimento norme.[/yellow]")
                break

        logger.info(f"Collected {len(applicable_law)} applicable laws")

        jurisdiction = Prompt.ask(
            "Giurisdizione (es. Tribunale di Milano)",
            default="") or None
        logger.info(f"Jurisdiction: {jurisdiction}")

        case_file = CaseFile(
            case_id=case_id,
            client=Party(name=client_name, role=client_role),
            parties=[Party(name=client_name, role=client_role)],
            facts=facts,
            jurisdiction=jurisdiction,
            applicable_law=applicable_law,
        )

        logger.info(f"Successfully created case file for {case_id}")
        return case_file

    except ValidationError as exc:
        logger.error(f"Validation error in case intake: {exc}")
        console.print(f"[red]Errore di validazione: {exc}[/red]")
        raise
    except KeyboardInterrupt:
        logger.info("Case intake interrupted by user")
        console.print("\n[yellow]Operazione annullata dall'utente.[/yellow]")
        raise
    except Exception as exc:
        logger.error(f"Unexpected error in case intake: {exc}", exc_info=True)
        console.print(f"[red]Errore inaspettato: {exc}[/red]")
        raise
