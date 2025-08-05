"""Utilities for detecting file encodings.

This module provides a :func:`detect_encoding` helper which tries to
identify the text encoding of a file.  The function focuses on the
encodings that are most common for Russian language documents and that
are specifically mentioned in the project specification:

* UTF-8
* Windows-1251
* UTF-16
* CP866

The implementation prefers the :mod:`chardet` library when it is
available but gracefully falls back to heuristic detection when the
library is missing.  The returned value is normalised to canonical
Python encoding names so that callers may safely pass it to ``open``.
"""
from __future__ import annotations

from pathlib import Path
from typing import Iterable, Tuple

try:  # pragma: no-cover - ``chardet`` might not be installed
    import chardet  # type: ignore
except ImportError:  # pragma: no-cover
    chardet = None  # type: ignore

# Encodings we officially support and their canonical return names.
_SUPPORTED_ENCODINGS: Iterable[Tuple[str, str]] = (
    ("utf-8", "utf-8"),
    ("utf-16", "utf-16"),
    ("cp1251", "windows-1251"),
    ("cp866", "cp866"),
)


def _cyrillic_score(text: str) -> float:
    """Return a score representing how much of *text* is Cyrillic."""
    if not text:
        return 0.0
    cyrillic = sum("\u0400" <= ch <= "\u04FF" for ch in text)
    printable = sum(ch.isprintable() for ch in text) or 1
    return cyrillic / printable


def detect_encoding(file_path: str | Path) -> str:
    """Guess the text encoding of ``file_path``.

    The function inspects the raw bytes of the file.  When ``chardet`` is
    available we delegate the detection to it; otherwise, we try a few
    candidate encodings and choose the one that yields the most Cyrillic
    characters, which works well for Russian texts.
    """
    path = Path(file_path)
    raw = path.read_bytes()

    # First try ``chardet`` if it is available.
    if chardet is not None:  # pragma: no branch - simple availability check
        result = chardet.detect(raw)
        enc = result.get("encoding") if isinstance(result, dict) else None
        if enc:
            enc = enc.lower()
            for candidate, returned in _SUPPORTED_ENCODINGS:
                if enc.startswith(candidate) or candidate in enc:
                    return returned

    # Fallback heuristic based on decoding attempts and Cyrillic ratio.
    best_score = -1.0
    best_encoding = "utf-8"
    for candidate, returned in _SUPPORTED_ENCODINGS:
        try:
            text = raw.decode(candidate)
        except UnicodeDecodeError:
            continue
        score = _cyrillic_score(text)
        if score > best_score:
            best_score = score
            best_encoding = returned
    return best_encoding
