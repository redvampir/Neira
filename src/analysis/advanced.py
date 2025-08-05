from __future__ import annotations

from dataclasses import dataclass
from typing import Dict

from src.memory import CharacterMemory
from src.models import Character


class SpellChecker:
    """Stub spell checker component."""

    def check(self, text: str) -> bool:  # pragma: no cover - simple stub
        return True


class LogicChecker:
    """Stub logic checker component."""

    def check(self, text: str) -> bool:  # pragma: no cover - simple stub
        return True


class ConsistencyChecker:
    """Stub consistency checker component."""

    def check(self, text: str) -> bool:  # pragma: no cover - simple stub
        return True


class StyleAnalyzer:
    """Stub style analyzer component."""

    def check(self, text: str) -> bool:  # pragma: no cover - simple stub
        return True


@dataclass
class AnalysisResult:
    """Result of running all analysis checks."""

    spelling: bool
    logic: bool
    consistency: bool
    style: bool


class AdvancedAnalyzer:
    """Run multiple analysis stages over generated text."""

    def __init__(self, memory: CharacterMemory | None = None) -> None:
        self.spell_checker = SpellChecker()
        self.logic_checker = LogicChecker()
        self.consistency_checker = ConsistencyChecker()
        self.style_analyzer = StyleAnalyzer()
        self.memory = memory or CharacterMemory()

    # ------------------------------------------------------------------
    def analyze_generation(self, generation: str) -> AnalysisResult:
        """Run all checks sequentially and return their combined result."""

        spelling = self.spell_checker.check(generation)
        logic = self.logic_checker.check(generation)
        consistency = self.consistency_checker.check(generation)
        consistency = consistency and self._check_character_consistency(generation)
        style = self.style_analyzer.check(generation)
        return AnalysisResult(
            spelling=spelling,
            logic=logic,
            consistency=consistency,
            style=style,
        )

    # ------------------------------------------------------------------
    def _check_character_consistency(self, generation: str) -> bool:
        """Ensure mentioned characters match stored appearance and traits."""

        text = generation.lower()
        characters: Dict[str, Character] = self.memory.get()  # type: ignore[assignment]
        for character in characters.values():
            name = character.name.lower()
            if name in text:
                if character.appearance and character.appearance.lower() not in text:
                    return False
                for trait in character.personality_traits:
                    if trait.lower() not in text:
                        return False
        return True


__all__ = ["AdvancedAnalyzer", "AnalysisResult"]
