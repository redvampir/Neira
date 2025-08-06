from __future__ import annotations

"""Utilities for language detection and basic text quality scoring."""


def detect_language(text: str) -> str:
    """Return an ISO language code for ``text``.

    The detection is deliberately lightweight and currently distinguishes
    between English and Russian based on the amount of Latin versus
    Cyrillic characters.  If neither script dominates, ``"unknown"`` is
    returned.
    """
    if not text:
        return "unknown"
    cyrillic = sum("\u0400" <= ch <= "\u04FF" for ch in text)
    latin = sum("a" <= ch.lower() <= "z" for ch in text)
    if cyrillic > latin:
        return "ru"
    if latin > cyrillic:
        return "en"
    return "unknown"


def quality_score(text: str) -> float:
    """Compute a naive quality score for ``text``.

    The score is the fraction of characters that are letters, digits or
    whitespace.  Well-formed sentences tend to score close to ``1``
    whereas random punctuation or gibberish results in lower values.
    """
    if not text:
        return 0.0
    letters = sum(ch.isalpha() for ch in text)
    digits = sum(ch.isdigit() for ch in text)
    spaces = sum(ch.isspace() for ch in text)
    return (letters + digits + spaces) / len(text)


__all__ = ["detect_language", "quality_score"]
