"""Simple glossary panel with search and auto-completion support."""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import Dict, List


@dataclass
class GlossaryPanel:
    """Store glossary terms and provide lookup helpers.

    The panel itself is a non-GUI placeholder that mimics the behaviour of
    a graphical glossary widget. It keeps a mapping of ``term -> definition``
    and exposes convenience methods for searching and autocompleting terms.
    """

    entries: Dict[str, str] = field(default_factory=dict)

    # ------------------------------------------------------------------
    def add_term(self, term: str, definition: str) -> None:
        """Add or update a glossary entry."""

        self.entries[term] = definition

    # ------------------------------------------------------------------
    def remove_term(self, term: str) -> None:
        """Remove a term from the glossary if it exists."""

        self.entries.pop(term, None)

    # ------------------------------------------------------------------
    def search(self, term: str) -> str | None:
        """Return the definition for ``term`` using case-insensitive match."""

        term_lower = term.lower()
        for key, value in self.entries.items():
            if key.lower() == term_lower:
                return value
        return None

    # ------------------------------------------------------------------
    def autocomplete(self, prefix: str) -> List[str]:
        """Return a list of terms starting with ``prefix`` (case-insensitive)."""

        prefix_lower = prefix.lower()
        return sorted(
            key for key in self.entries.keys() if key.lower().startswith(prefix_lower)
        )
