from __future__ import annotations

from src.utils.language_adapter import adapt_request, adapt_response


class DummyTranslator:
    """Minimal translator used to simulate external translation service."""

    def translate(self, text: str, src: str, dest: str) -> object:  # pragma: no cover - simple utility
        return type("T", (), {"text": f"{text}-{src}->{dest}"})()


def test_adapt_request_detects_language_and_translates():
    lang, translated = adapt_request("Hello", translator=DummyTranslator())
    assert lang == "en"
    assert translated == "Hello-en->ru"


def test_adapt_response_translates_back_to_original_language():
    result = adapt_response("Привет", "en", translator=DummyTranslator())
    assert result == "Привет-ru->en"


def test_adapt_request_without_translator_returns_original_text():
    lang, translated = adapt_request("Привет")
    assert lang == "ru"
    assert translated == "Привет"
