from __future__ import annotations

"""Simple heuristics-based spam detection."""

import re

# A small set of common spam keywords and phrases.
_SPAM_PATTERNS = [
    r"free\b",
    r"buy\s+now",
    r"click\s+here",
    r"subscribe\s+now",
    r"win\s+\w+",
    r"earn\s+money",
    r"credit\s+card",
    r"guaranteed",
]


def is_spam(text: str) -> bool:
    """Return ``True`` if ``text`` is considered spam.

    The implementation relies on a few simple heuristics: if any of the
    predefined patterns is found or if the text contains an unusual amount of
    exclamation marks, it is flagged as spam.  While rudimentary, this approach
    is sufficient for basic filtering and is easily extendable.
    """
    if not text:
        return False

    lowered = text.lower()
    for pattern in _SPAM_PATTERNS:
        if re.search(pattern, lowered):
            return True

    # Excessive punctuation is another common indicator of spam.
    if lowered.count("!") >= 3:
        return True

    return False


__all__ = ["is_spam"]
