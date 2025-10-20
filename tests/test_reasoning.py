import pytest

from counsel_ai.models import CaseFile, Party
from counsel_ai.reasoning import (
    _analyze_applicable_law,
    _analyze_facts,
    build_reasoning,
    draft_opinion,
)


class TestAnalyzeFacts:
    def test_analyze_facts_inadempimento(self):
        facts = [
            "Il debitore non ha pagato la rata",
            "Mancato pagamento della somma dovuta",
        ]
        result = _analyze_facts(facts)
        assert "inadempimento" in result[0].lower()

    def test_analyze_facts_danno(self):
        facts = ["Il cliente ha subito un danno", "Lesione del diritto"]
        result = _analyze_facts(facts)
        assert "danno" in result[0].lower()

    def test_analyze_facts_contratto(self):
        facts = ["Firmato un contratto", "Accordo tra le parti"]
        result = _analyze_facts(facts)
        assert "contrattuali" in result[0].lower()

    def test_analyze_facts_empty(self):
        facts = []
        result = _analyze_facts(facts)
        assert len(result) == 0


class TestAnalyzeApplicableLaw:
    def test_analyze_applicable_law_art_1218(self):
        laws = ["art. 1218 c.c.", "altra norma"]
        result = _analyze_applicable_law(laws)
        assert "art. 1218" in result[0]

    def test_analyze_applicable_law_cassazione(self):
        laws = ["Cass. Civ. 30574/2022", "altra norma"]
        result = _analyze_applicable_law(laws)
        assert "legittimità" in result[0]

    def test_analyze_applicable_law_tribunale(self):
        laws = ["Trib. Milano 123/2023", "altra norma"]
        result = _analyze_applicable_law(laws)
        assert "merito" in result[0]


class TestBuildReasoning:
    def test_build_reasoning_success(self):
        client = Party(name="Test Client", role="Cliente")
        case = CaseFile(
            case_id="HT-2025-0001",
            client=client,
            facts=["Il debitore non ha pagato"],
            applicable_law=["art. 1218 c.c."],
        )

        result = build_reasoning(case)

        assert result.root_id == "root"
        assert len(result.nodes) == 1
        assert "responsabilità contrattuale" in result.summary.lower()

    def test_build_reasoning_no_facts(self):
        client = Party(name="Test Client", role="Cliente")
        case = CaseFile(
            case_id="HT-2025-0001", client=client, facts=[], applicable_law=[]
        )

        result = build_reasoning(case)

        assert result.root_id == "root"
        assert "ulteriore istruttoria" in result.summary

    def test_build_reasoning_error_handling(self):
        # Test with invalid case data that might cause errors
        client = Party(name="Test Client", role="Cliente")
        case = CaseFile(
            case_id="HT-2025-0001",
            client=client,
            facts=["Test fact"],
            applicable_law=["art. 1218 c.c."],
        )

        result = build_reasoning(case)

        # Should return a valid reasoning tree even if there are issues
        assert result.root_id is not None
        assert result.summary is not None


class TestDraftOpinion:
    def test_draft_opinion_success(self):
        client = Party(name="Test Client", role="Cliente")
        case = CaseFile(
            case_id="HT-2025-0001",
            client=client,
            facts=["Il debitore non ha pagato"],
            applicable_law=["art. 1218 c.c."],
        )

        reasoning = build_reasoning(case)
        opinion = draft_opinion(case, reasoning)

        assert opinion.case_id == "HT-2025-0001"
        assert "HT-2025-0001" in opinion.title
        assert len(opinion.recommendations) > 0
        assert len(opinion.citations) >= 0

    def test_draft_opinion_error_handling(self):
        client = Party(name="Test Client", role="Cliente")
        case = CaseFile(
            case_id="HT-2025-0001", client=client, facts=[], applicable_law=[]
        )

        reasoning = build_reasoning(case)
        opinion = draft_opinion(case, reasoning)

        # Should return a valid opinion even with minimal data
        assert opinion.case_id == "HT-2025-0001"
        assert len(opinion.recommendations) > 0
