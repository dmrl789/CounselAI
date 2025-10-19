from __future__ import annotations
from typing import List
from datetime import datetime
from .models import CaseFile, ReasoningNode, ReasoningTree, Opinion


CANONICAL_CITATIONS = {
    "art. 1218 c.c.": "Codice Civile, Art. 1218 - Responsabilità del debitore",
    "Cass. Civ. 30574/2022": "Corte di Cassazione, Sez. III, Sent. n. 30574/2022",
}


def build_reasoning(case: CaseFile) -> ReasoningTree:
    root = ReasoningNode(
        id="root",
        claim="Valutazione della responsabilità contrattuale e rimedi disponibili",
        supports=[
            "Analisi dei fatti forniti",
            "Applicazione delle norme e dei precedenti indicati",
        ],
        citations=[c for c in case.applicable_law if c in CANONICAL_CITATIONS],
        timestamp=datetime.utcnow(),
    )

    nodes: List[ReasoningNode] = [root]

    summary_parts: List[str] = []
    if any("inadempimento" in f.lower() or "mancato pagamento" in f.lower() for f in case.facts):
        summary_parts.append("Sussistono profili di inadempimento ex art. 1218 c.c.")

    if not summary_parts:
        summary_parts.append("Necessaria ulteriore istruttoria dei fatti")

    summary = "; ".join(summary_parts)

    return ReasoningTree(root_id=root.id, nodes=nodes, summary=summary)


def draft_opinion(case: CaseFile, reasoning: ReasoningTree) -> Opinion:
    recommendations: List[str] = []
    citations = list({c for node in reasoning.nodes for c in node.citations})

    if "Sussistono profili di inadempimento" in reasoning.summary:
        recommendations.append("Valutare diffida ad adempiere e successiva azione per risoluzione/danni")
    else:
        recommendations.append("Raccogliere documentazione integrativa e chiarire i fatti rilevanti")

    title = f"Parere legale su pratica {case.case_id}"
    summary = reasoning.summary

    return Opinion(
        case_id=case.case_id,
        title=title,
        summary=summary,
        recommendations=recommendations,
        citations=citations,
    )
