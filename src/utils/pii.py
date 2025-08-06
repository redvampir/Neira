"""Utilities for redacting personally identifiable information (PII)."""

from __future__ import annotations

import re

_EMAIL_RE = re.compile(r"[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+")
_PHONE_RE = re.compile(r"\b(?:\+?\d{1,3}[-.\s]?)?(?:\(?\d{3}\)?[-.\s]?){1,2}\d{4}\b")


def redact_pii(text: str) -> str:
    """Replace common PII like emails and phone numbers with ``[REDACTED]``.

    This lightweight implementation uses regular expressions and is not meant
    to be exhaustive but provides a reasonable safeguard for tests and simple
    usage.
    """
    text = _EMAIL_RE.sub("[REDACTED]", text)
    text = _PHONE_RE.sub("[REDACTED]", text)
    return text


__all__ = ["redact_pii"]
