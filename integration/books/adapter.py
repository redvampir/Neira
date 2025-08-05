"""Adapter utilities for turning books into RPG material."""

from __future__ import annotations

from typing import Any, Dict, Iterable, List

from .extractor import SettingExtractor


class BookToRPGAdapter:
    """Convert book information into RPG-friendly structures."""

    def __init__(self, extractor: SettingExtractor | None = None) -> None:
        self.extractor = extractor or SettingExtractor()

    def create_campaign_from_book(self, book: Dict[str, Any], target_system: str) -> Dict[str, Any]:
        """Create a campaign dictionary from book data."""
        content = book.get("content", "")
        characters = book.get("characters", [])
        campaign = {
            "system": target_system,
            "title": book.get("title", "Untitled"),
            "world_rules": self.extractor.extract_world_rules(content),
            "npcs": self.extractor.create_npcs_from_characters(characters),
            "locations": self.extractor.generate_locations(content),
            "plot_hooks": self.extractor.extract_plot_hooks(content),
        }
        return campaign

    def balance_characters_for_game(
        self, characters: Iterable[Dict[str, Any]]
    ) -> List[Dict[str, Any]]:
        """Balance character levels by setting them to the average."""
        characters = list(characters)
        if not characters:
            return []
        average = sum(c.get("level", 1) for c in characters) / len(characters)
        balanced: List[Dict[str, Any]] = []
        for character in characters:
            new_char = dict(character)
            new_char["level"] = int(average)
            balanced.append(new_char)
        return balanced

    def generate_adventures(self, plot_elements: Iterable[str]) -> List[Dict[str, str]]:
        """Generate simple adventure descriptions from plot elements."""
        adventures = []
        for element in plot_elements:
            adventures.append(
                {
                    "title": element[:50],
                    "summary": element,
                    "encounters": [f"Encounter: {element}"],
                }
            )
        return adventures
