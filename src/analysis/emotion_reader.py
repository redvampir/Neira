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
            "fear": {
                "fear",
                "scared",
                "terrified",
                "afraid",
                "fright",
                "nervous",
                "anxious",
            },
        }
        # Негативные частицы для учета отрицаний.
        self.negations: set[str] = {"not", "no", "never", "n't"}

    def analyze_text(self, text: str) -> Dict[str, float]:
        """Возвращает оценки эмоций в диапазоне [0, 1] для текста.

        Ключевые слова каждой эмоции подсчитываются и нормируются, при
        этом игнорируются слова, перед которыми стоит отрицание.
        """
        words = re.findall(r"\b\w+\b", text.lower())
        raw_scores: Dict[str, float] = {emotion: 0.0 for emotion in self.lexicon}
        for idx, word in enumerate(words):
            if idx > 0 and words[idx - 1] in self.negations:
                continue
            for emotion, keywords in self.lexicon.items():
                if word in keywords:
                    raw_scores[emotion] += 1
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

