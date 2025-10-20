from __future__ import annotations
import logging
from typing import List
from datetime import datetime, timezone
from .models import CaseFile, ReasoningNode, ReasoningTree, Opinion

logger = logging.getLogger(__name__)


CANONICAL_CITATIONS = {
    "art. 1218 c.c.": "Codice Civile, Art. 1218 - Responsabilità del debitore",
    "Cass. Civ. 30574/2022": "Corte di Cassazione, Sez. III, Sent. n. 30574/2022",
}


def build_reasoning(case: CaseFile) -> ReasoningTree:
    """Build reasoning tree from case file with error handling"""
    try:
        logger.info(f"Building reasoning for case {case.case_id}")
        
        # Validate case has required data
        if not case.facts:
            logger.warning("No facts provided in case file")
        if not case.applicable_law:
            logger.warning("No applicable law provided in case file")
        
        root = ReasoningNode(
            id="root",
            claim="Valutazione della responsabilità contrattuale e rimedi disponibili",
            supports=[
                "Analisi dei fatti forniti",
                "Applicazione delle norme e dei precedenti indicati",
            ],
            citations=[c for c in case.applicable_law if c in CANONICAL_CITATIONS],
            timestamp=datetime.now(timezone.utc),
        )

        nodes: List[ReasoningNode] = [root]

        summary_parts: List[str] = []
        
        # Analyze facts for specific legal issues
        fact_analysis = _analyze_facts(case.facts)
        summary_parts.extend(fact_analysis)
        
        # Add law-specific analysis
        law_analysis = _analyze_applicable_law(case.applicable_law)
        summary_parts.extend(law_analysis)

        if not summary_parts:
            summary_parts.append("Necessaria ulteriore istruttoria dei fatti")

        summary = "; ".join(summary_parts)
        logger.info(f"Generated reasoning summary: {summary}")

        return ReasoningTree(root_id=root.id, nodes=nodes, summary=summary)
        
    except Exception as e:
        logger.error(f"Error building reasoning: {e}", exc_info=True)
        # Return a basic reasoning tree in case of error
        return ReasoningTree(
            root_id="error",
            nodes=[ReasoningNode(
                id="error",
                claim="Errore nell'analisi del caso",
                supports=["Errore tecnico"],
                citations=[],
                timestamp=datetime.now(timezone.utc),
            )],
            summary="Errore nell'analisi del caso. Consultare i log per dettagli."
        )


def _analyze_facts(facts: List[str]) -> List[str]:
    """Analyze facts for legal issues"""
    analysis = []
    
    for fact in facts:
        fact_lower = fact.lower()
        if any(keyword in fact_lower for keyword in ["inadempimento", "mancato pagamento", "ritardo"]):
            analysis.append("Sussistono profili di inadempimento ex art. 1218 c.c.")
        elif any(keyword in fact_lower for keyword in ["danno", "lesione", "pregiudizio"]):
            analysis.append("Presenza di elementi di danno risarcibile")
        elif any(keyword in fact_lower for keyword in ["contratto", "accordo", "convenzione"]):
            analysis.append("Rilevante presenza di accordi contrattuali")
    
    return analysis


def _analyze_applicable_law(applicable_law: List[str]) -> List[str]:
    """Analyze applicable law for specific provisions"""
    analysis = []
    
    for law in applicable_law:
        if "art. 1218" in law:
            analysis.append("Applicazione dell'art. 1218 c.c. per responsabilità contrattuale")
        elif "Cass." in law or "Cassazione" in law:
            analysis.append("Rilevanza della giurisprudenza di legittimità")
        elif "Trib." in law or "Tribunale" in law:
            analysis.append("Rilevanza della giurisprudenza di merito")
    
    return analysis


def draft_opinion(case: CaseFile, reasoning: ReasoningTree) -> Opinion:
    """Draft legal opinion from case and reasoning with error handling"""
    try:
        logger.info(f"Drafting opinion for case {case.case_id}")
        
        recommendations: List[str] = []
        citations = list({c for node in reasoning.nodes for c in node.citations})
        
        # Generate recommendations based on reasoning
        recommendations.extend(_generate_recommendations(reasoning.summary, case.facts))
        
        # Ensure we have at least one recommendation
        if not recommendations:
            recommendations.append("Raccogliere documentazione integrativa e chiarire i fatti rilevanti")

        title = f"Parere legale su pratica {case.case_id}"
        summary = reasoning.summary

        opinion = Opinion(
            case_id=case.case_id,
            title=title,
            summary=summary,
            recommendations=recommendations,
            citations=citations,
        )
        
        logger.info(f"Successfully drafted opinion with {len(recommendations)} recommendations")
        return opinion
        
    except Exception as e:
        logger.error(f"Error drafting opinion: {e}", exc_info=True)
        # Return a basic opinion in case of error
        return Opinion(
            case_id=case.case_id,
            title=f"Parere legale su pratica {case.case_id} (ERRORE)",
            summary="Errore nella generazione del parere. Consultare i log per dettagli.",
            recommendations=["Consultare un legale per assistenza"],
            citations=[],
        )


def _generate_recommendations(summary: str, facts: List[str]) -> List[str]:
    """Generate recommendations based on summary and facts"""
    recommendations = []
    
    if "inadempimento" in summary.lower():
        recommendations.append("Valutare diffida ad adempiere e successiva azione per risoluzione/danni")
        recommendations.append("Documentare l'inadempimento con prova scritta")
    
    if "danno" in summary.lower():
        recommendations.append("Quantificare il danno subito con documentazione probatoria")
        recommendations.append("Valutare azione per risarcimento danni")
    
    if "contratto" in summary.lower():
        recommendations.append("Verificare la validità e l'efficacia del contratto")
        recommendations.append("Analizzare le clausole contrattuali applicabili")
    
    if not recommendations:
        recommendations.append("Raccogliere documentazione integrativa e chiarire i fatti rilevanti")
        recommendations.append("Consultare la normativa applicabile al caso specifico")
    
    return recommendations
