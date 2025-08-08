from __future__ import annotations

import json
import random
from pathlib import Path

from src.plugins import Plugin


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

    def on_gap_analysis(self, draft: str, gaps) -> None:  # pragma: no cover - trivial
        if not self.cards:
            return
        card = random.choice(self.cards)
        print(f"Inspiration: {card}")
