"""Utilities for extracting RPG settings from book content."""

from __future__ import annotations

import re
from typing import Any, Dict, Iterable, List


class SettingExtractor:
    """Extract setting information from books for RPG usage."""

    _RULE_KEYWORDS = ("must", "shall", "never", "always", "law")
    _PLOT_HOOK_KEYWORDS = (
        "mysterious",
        "quest",
        "secret",
        "threat",
        "danger",
        "murder",
        "prophecy",
        "legend",
    )
    _LOCATION_PATTERNS = re.compile(
        r"\b(?:city|town|village|forest|castle|kingdom|mountain|lake|sea|desert|island)\s+([A-Z][a-zA-Z]+)"
    )

    def extract_world_rules(self, book_content: str) -> List[str]:
        """Return sentences that describe world rules.

        Sentences containing certain keywords are considered rules.
        """
        sentences = re.split(r"[.!?]", book_content)
        rules = [
            sentence.strip()
            for sentence in sentences
            if any(key in sentence.lower() for key in self._RULE_KEYWORDS)
        ]
        return [rule for rule in rules if rule]

    def create_npcs_from_characters(
        self, characters: Iterable[Any]
    ) -> List[Dict[str, Any]]:
        """Convert character representations into NPC dictionaries."""
        npcs: List[Dict[str, Any]] = []
        for character in characters:
            if isinstance(character, dict):
                npc = {"name": character.get("name", "Unnamed"), "role": "npc"}
                if "traits" in character:
                    npc["traits"] = list(character["traits"])
            else:
                npc = {"name": str(character), "role": "npc"}
            npcs.append(npc)
        return npcs

    def generate_locations(self, book_content: str) -> List[str]:
        """Identify location names from the content."""
        return list(set(self._LOCATION_PATTERNS.findall(book_content)))

    def extract_plot_hooks(self, book_content: str) -> List[str]:
        """Find sentences that could serve as plot hooks."""
        sentences = re.split(r"[.!?]", book_content)
        hooks = [
            sentence.strip()
            for sentence in sentences
            if any(key in sentence.lower() for key in self._PLOT_HOOK_KEYWORDS)
        ]
        return [hook for hook in hooks if hook]
