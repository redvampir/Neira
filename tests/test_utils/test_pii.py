"""Tests for PII redaction utilities."""

from src.utils.pii import redact_pii
from src.search.api_client import SearchAPIClient
from src.memory import MemoryIndex


def test_redacts_email_and_phone() -> None:
    text = "Contact: john.doe@example.com or 123-456-7890."
    redacted = redact_pii(text)
    assert "john.doe@example.com" not in redacted
    assert "123-456-7890" not in redacted
    assert redacted.count("[REDACTED]") == 2


def test_search_and_update_applies_redaction() -> None:
    mem = MemoryIndex()

    def fake_fetch(query: str, limit: int):
        return [
            {
                "url": "https://example.com",
                "snippet": "Call me at 123-456-7890.",
            }
        ]

    client = SearchAPIClient(memory=mem, fetcher=fake_fetch)
    client.search_and_update("test")

    stored_fact = next(iter(mem.cold_storage))
    assert stored_fact == "Call me at [REDACTED]"
    assert "123-456-7890" not in stored_fact
