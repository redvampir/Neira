from __future__ import annotations

import json
import random
import re
from pathlib import Path

from src.plugins import Plugin
from src.memory.idea_catalog import IdeaCatalog


class InspirationCardsPlugin(Plugin):
    """Show a random inspiration card during gap analysis."""

    def __init__(self) -> None:
        data_path = (
            Path(__file__).resolve().parents[1] / "data" / "inspiration_cards.json"
        )
        try:
            with data_path.open("r", encoding="utf-8") as f:
                self.cards: list[str] = json.load(f)
        except Exception:
            self.cards = []

        self.catalog = IdeaCatalog()

    # ------------------------------------------------------------------
    def _resolve_references(self, text: str) -> str:
        """Replace ``{{key}}`` placeholders with catalog entries."""

        def repl(match: re.Match[str]) -> str:
            key = match.group(1).strip()
            value = self.catalog.get(key)
            return str(value) if value is not None else match.group(0)

        return re.sub(r"\{\{([^{}]+)\}\}", repl, text)

    def on_gap_analysis(self, draft: str, gaps) -> None:  # pragma: no cover - trivial
        if not self.cards:
            return
        card = random.choice(self.cards)
        card = self._resolve_references(card)
        print(f"Inspiration: {card}")
