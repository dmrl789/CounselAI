import json
import tempfile
from pathlib import Path

from counsel_ai.audit import append_record, verify_ledger_integrity


class TestAuditSystem:
    def setup_method(self):
        # Create a temporary directory for testing
        self.temp_dir = tempfile.mkdtemp()
        self.ledger_file = Path(self.temp_dir) / "ledger.jsonl"

        # Mock the global variables
        import counsel_ai.audit as audit_module

        self.original_audit_dir = audit_module.AUDIT_DIR
        self.original_ledger_file = audit_module.LEDGER_FILE

        audit_module.AUDIT_DIR = Path(self.temp_dir)
        audit_module.LEDGER_FILE = self.ledger_file

    def teardown_method(self):
        # Restore original values
        import counsel_ai.audit as audit_module

        audit_module.AUDIT_DIR = self.original_audit_dir
        audit_module.LEDGER_FILE = self.original_ledger_file

    def test_append_record_first_record(self):
        payload = {"test": "data"}
        chain_hash = append_record("test_action", payload)

        assert self.ledger_file.exists()

        with open(self.ledger_file) as f:
            lines = f.readlines()
            assert len(lines) == 1

            record = json.loads(lines[0])
            assert record["action"] == "test_action"
            assert record["data_hash"] is not None
            assert record["prev_hash"] == "0" * 64
            assert record["chain_hash"] == chain_hash

    def test_append_record_chain_integrity(self):
        # Add first record
        payload1 = {"test": "data1"}
        chain_hash1 = append_record("action1", payload1)

        # Add second record
        payload2 = {"test": "data2"}
        chain_hash2 = append_record("action2", payload2)

        with open(self.ledger_file) as f:
            lines = f.readlines()
            assert len(lines) == 2

            # Check first record
            record1 = json.loads(lines[0])
            assert record1["chain_hash"] == chain_hash1

            # Check second record
            record2 = json.loads(lines[1])
            assert record2["prev_hash"] == chain_hash1
            assert record2["chain_hash"] == chain_hash2

    def test_verify_ledger_integrity_valid(self):
        # Add some records
        append_record("action1", {"data": "1"})
        append_record("action2", {"data": "2"})

        # Verify integrity
        assert verify_ledger_integrity() is True

    def test_verify_ledger_integrity_corrupted(self):
        # Add a record
        append_record("action1", {"data": "1"})

        # Corrupt the file
        with open(self.ledger_file, "w") as f:
            f.write("invalid json\n")

        # Verify integrity should fail
        assert verify_ledger_integrity() is False

    def test_verify_ledger_integrity_empty_file(self):
        # Create empty file
        self.ledger_file.touch()

        # Verify integrity should pass
        assert verify_ledger_integrity() is True

    def test_verify_ledger_integrity_nonexistent_file(self):
        # File doesn't exist
        assert verify_ledger_integrity() is True

    def test_append_record_large_payload(self):
        # Test with large payload
        large_payload = {"data": "x" * 10000}
        chain_hash = append_record("large_action", large_payload)

        assert chain_hash is not None
        assert self.ledger_file.exists()

    def test_append_record_special_characters(self):
        # Test with special characters in payload
        payload = {"special": "chars: !@#$%^&*()_+-=[]{}|;':\",./<>?"}
        chain_hash = append_record("special_action", payload)

        assert chain_hash is not None

        # Verify the record was written correctly
        with open(self.ledger_file) as f:
            record = json.loads(f.read().strip())
            assert record["action"] == "special_action"
            assert (
                record["payload"]["special"] == "chars: !@#$%^&*()_+-=[]{}|;':\",./<>?"
            )
