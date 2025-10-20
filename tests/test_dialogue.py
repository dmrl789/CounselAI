from unittest.mock import patch

import pytest

from counsel_ai.dialogue import _ask_non_empty, interactive_intake


class TestAskNonEmpty:
    def test_ask_non_empty_valid_input(self):
        with patch("counsel_ai.dialogue.Prompt.ask", return_value="Valid input"):
            result = _ask_non_empty("Test prompt")
            assert result == "Valid input"

    def test_ask_non_empty_empty_input_retry(self):
        with patch("counsel_ai.dialogue.Prompt.ask", side_effect=["", "Valid input"]):
            result = _ask_non_empty("Test prompt")
            assert result == "Valid input"

    def test_ask_non_empty_max_attempts(self):
        with patch("counsel_ai.dialogue.Prompt.ask", return_value=""):
            with pytest.raises(ValueError, match="Maximum attempts exceeded"):
                _ask_non_empty("Test prompt", max_attempts=2)

    def test_ask_non_empty_keyboard_interrupt(self):
        with patch("counsel_ai.dialogue.Prompt.ask", side_effect=KeyboardInterrupt):
            with pytest.raises(KeyboardInterrupt):
                _ask_non_empty("Test prompt")


class TestInteractiveIntake:
    @patch("counsel_ai.dialogue.Prompt.ask")
    def test_interactive_intake_success(self, mock_prompt):
        # Mock all the prompts
        mock_prompt.side_effect = [
            "HT-2025-0001",  # case_id
            "Test Client",  # client_name
            "Cliente",  # client_role
            "Fact 1",  # first fact
            "Fact 2",  # second fact
            "",  # empty fact (end)
            "art. 1218 c.c.",  # first law
            "",  # empty law (end)
            "Tribunale di Milano",  # jurisdiction
        ]

        result = interactive_intake()

        assert result.case_id == "HT-2025-0001"
        assert result.client.name == "Test Client"
        assert result.client.role == "Cliente"
        assert len(result.facts) == 2
        assert len(result.applicable_law) == 1
        assert result.jurisdiction == "Tribunale di Milano"

    @patch("counsel_ai.dialogue.Prompt.ask")
    def test_interactive_intake_with_existing_case_id(self, mock_prompt):
        mock_prompt.side_effect = [
            "Test Client",  # client_name
            "Cliente",  # client_role
            "Fact 1",  # first fact
            "",  # empty fact (end)
            "art. 1218 c.c.",  # first law
            "",  # empty law (end)
            "Tribunale di Milano",  # jurisdiction
        ]

        result = interactive_intake("HT-2025-0001")

        assert result.case_id == "HT-2025-0001"
        assert result.client.name == "Test Client"

    @patch("counsel_ai.dialogue.Prompt.ask")
    def test_interactive_intake_keyboard_interrupt(self, mock_prompt):
        mock_prompt.side_effect = KeyboardInterrupt()

        with pytest.raises(KeyboardInterrupt):
            interactive_intake()

    @patch("counsel_ai.dialogue.Prompt.ask")
    def test_interactive_intake_validation_error(self, mock_prompt):
        mock_prompt.side_effect = [
            "",  # empty case_id should cause validation error
            "",  # second attempt
            "",  # third attempt - should raise ValueError
        ]

        with pytest.raises(ValueError, match="Maximum attempts exceeded"):
            interactive_intake()
