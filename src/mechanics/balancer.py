from __future__ import annotations

"""Utilities for keeping game encounters balanced."""

from typing import Dict, List, Sequence


class GameBalancer:
    """Track encounter difficulty and spotlight distribution."""

    def __init__(self) -> None:
        self.difficulties: List[int] = []
        self.challenge_level: float = 0.0
        self.participation: Dict[str, int] = {}

    def monitor_encounter_difficulty(self, difficulty: int) -> float:
        """Record *difficulty* and return the rolling average."""

        self.difficulties.append(int(difficulty))
        return sum(self.difficulties) / len(self.difficulties)

    def adjust_challenge_level(self, target_average: float) -> float:
        """Adjust ``challenge_level`` toward ``target_average`` and return it."""

        if not self.difficulties:
            self.challenge_level = float(target_average)
        else:
            current = sum(self.difficulties) / len(self.difficulties)
            self.challenge_level += target_average - current
        return self.challenge_level

    def ensure_spotlight_distribution(self, actors: Sequence[str]) -> bool:
        """Track *actors* participation and ensure it stays roughly even.

        Returns ``True`` if the difference between the most and least active
        actors is at most one; otherwise ``False``.
        """

        for actor in actors:
            self.participation[actor] = self.participation.get(actor, 0) + 1
        counts = list(self.participation.values())
        return max(counts) - min(counts) <= 1 if counts else True
