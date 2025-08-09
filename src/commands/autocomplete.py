from __future__ import annotations

"""Autocomplete database and suggestion helpers.

The module maintains a small in-memory table describing available command
``tags`` together with their expected argument hints and the programming
languages where they are applicable.  Callers can query
:func:`get_suggestions` to retrieve context aware completions which can be
further filtered by file type and the current editor mode.
"""

from dataclasses import dataclass
from typing import Dict, List


@dataclass(frozen=True)
class TagHint:
    """Metadata describing a single autocomplete entry."""

    arguments: List[str]
    languages: List[str]
    modes: List[str]


# ---------------------------------------------------------------------------
# Core database of known tags.  The dataset is intentionally tiny – it merely
# provides enough structure for unit tests exercising the autocomplete logic.
TAG_HINTS: Dict[str, TagHint] = {
    "@todo": TagHint(
        arguments=["message"],
        languages=["python", "markdown"],
        modes=["code_editor", "book_editor"],
    ),
    "@style": TagHint(
        arguments=["selector", "property"],
        languages=["css"],
        modes=["code_editor", "visual_programming"],
    ),
    "@node": TagHint(
        arguments=["type"],
        languages=["python"],
        modes=["visual_programming"],
    ),
}

# Derive a flat list of languages for quick lookup or display purposes.
LANGUAGES: List[str] = sorted({lang for hint in TAG_HINTS.values() for lang in hint.languages})


def get_suggestions(
    prefix: str,
    *,
    file_type: str | None = None,
    mode: str | None = None,
) -> List[str]:
    """Return autocomplete suggestions matching ``prefix``.

    Parameters
    ----------
    prefix:
        Partial tag typed by the user.  Matching is case insensitive.
    file_type:
        Optional hint specifying the language of the file currently being
        edited.  When provided, only tags advertising that language are
        returned.
    mode:
        Name of the active editor (e.g. ``"code_editor"``).  When given, only
        tags supporting the mode are included in the results.
    """

    prefix = prefix.lower()
    suggestions: List[str] = []
    for tag, hint in TAG_HINTS.items():
        if not tag.startswith(prefix):
            continue
        if file_type and file_type not in hint.languages:
            continue
        if mode and mode not in hint.modes:
            continue
        suggestion = tag
        if hint.arguments:
            suggestion += " " + " ".join(hint.arguments)
        suggestions.append(suggestion)
    return sorted(suggestions)


__all__ = ["TagHint", "TAG_HINTS", "LANGUAGES", "get_suggestions"]
