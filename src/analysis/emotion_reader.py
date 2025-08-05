"""
Читатель эмоций - чувствую настроение текста.
"""

from __future__ import annotations

import re
from collections import defaultdict
from typing import Dict


class EmotionReader:
    """Я улавливаю эмоциональный оттенок повествования."""

    def __init__(self) -> None:
        # Простая лексика эмоций для эвристического анализа.
        self.lexicon: Dict[str, set[str]] = {
            "joy": {"happy", "joy", "joyful", "delight", "pleased", "smile"},
            "sadness": {"sad", "unhappy", "sorrow", "cry", "miserable"},
            "anger": {"angry", "mad", "furious", "rage", "irate"},
            "fear": {"fear", "scared", "terrified", "afraid", "fright"},
        }

    def analyze_text(self, text: str) -> Dict[str, float]:
        """Возвращает оценки эмоций в диапазоне [0, 1] для текста."""
        words = re.findall(r"\b\w+\b", text.lower())
        raw_scores = {
            emotion: sum(1 for w in words if w in keywords)
            for emotion, keywords in self.lexicon.items()
        }
        return self.scale_scores(raw_scores)

    @staticmethod
    def scale_scores(scores: Dict[str, float]) -> Dict[str, float]:
        """Масштабирует оценки так, что максимальная равна 1."""
        if not scores:
            return {}
        max_score = max(scores.values()) or 1.0
        return {emotion: value / max_score for emotion, value in scores.items()}

    @staticmethod
    def combine_scores(*score_dicts: Dict[str, float]) -> Dict[str, float]:
        """Комбинирует несколько наборов оценок, суммируя значения."""
        combined: defaultdict[str, float] = defaultdict(float)
        for d in score_dicts:
            for emotion, value in d.items():
                combined[emotion] += value
        return dict(combined)

