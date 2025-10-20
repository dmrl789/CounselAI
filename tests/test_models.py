import pytest

from counsel_ai.models import CaseFile, Opinion, Party, ReasoningNode, ReasoningTree


class TestParty:
    def test_party_creation(self):
        party = Party(name="Test Client", role="Cliente")
        assert party.name == "Test Client"
        assert party.role == "Cliente"

    def test_party_validation(self):
        with pytest.raises(ValueError):
            Party(name="", role="Cliente")

        with pytest.raises(ValueError):
            Party(name="Test", role="InvalidRole")


class TestCaseFile:
    def test_case_file_creation(self):
        client = Party(name="Test Client", role="Cliente")
        case = CaseFile(
            case_id="HT-2025-0001",
            client=client,
            facts=["Test fact 1", "Test fact 2"],
            applicable_law=["art. 1218 c.c."],
        )
        assert case.case_id == "HT-2025-0001"
        assert len(case.facts) == 2
        assert len(case.applicable_law) == 1

    def test_case_file_validation(self):
        with pytest.raises(ValueError):
            CaseFile(case_id="", client=Party(name="Test", role="Cliente"))


class TestReasoningNode:
    def test_reasoning_node_creation(self):
        node = ReasoningNode(
            id="test-node",
            claim="Test claim",
            supports=["Support 1", "Support 2"],
            citations=["Citation 1"],
        )
        assert node.id == "test-node"
        assert node.claim == "Test claim"
        assert len(node.supports) == 2
        assert len(node.citations) == 1


class TestReasoningTree:
    def test_reasoning_tree_creation(self):
        root = ReasoningNode(id="root", claim="Root claim", supports=[], citations=[])
        tree = ReasoningTree(root_id="root", nodes=[root], summary="Test summary")
        assert tree.root_id == "root"
        assert len(tree.nodes) == 1
        assert tree.summary == "Test summary"


class TestOpinion:
    def test_opinion_creation(self):
        opinion = Opinion(
            case_id="HT-2025-0001",
            title="Test Opinion",
            summary="Test summary",
            recommendations=["Recommendation 1"],
            citations=["Citation 1"],
        )
        assert opinion.case_id == "HT-2025-0001"
        assert opinion.title == "Test Opinion"
        assert len(opinion.recommendations) == 1
        assert len(opinion.citations) == 1
