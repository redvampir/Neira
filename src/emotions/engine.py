from __future__ import annotations

"""Simple emotional engine for the Neyra character."""

from dataclasses import dataclass, field
from typing import List


@dataclass
class NeyraEmotions:
    """Track emotional state and derive desires for the character.

    Parameters represent floating point values between 0 and 1.  The
    class keeps a history of recent successes and failures to adjust the
    overall mood dynamically.
    """

    mood: float = 0.5
    curiosity: float = 0.5
    creativity: float = 0.5
    helpfulness: float = 0.5
    growth_desire: float = 0.5
    recent_successes: List[str] = field(default_factory=list)
    recent_failures: List[str] = field(default_factory=list)
    history_limit: int = 10

    def update_mood_from_task(self, task: str, success: bool) -> None:
        """Update mood after completing a task.

        Args:
            task: Name or description of the task.
            success: ``True`` if the task was successful, ``False`` otherwise.
        """

        if success:
            self.recent_successes.append(task)
            delta = 0.05
        else:
            self.recent_failures.append(task)
            delta = -0.05

        self.recent_successes = self.recent_successes[-self.history_limit :]
        self.recent_failures = self.recent_failures[-self.history_limit :]

        trend = 0.01 * (len(self.recent_successes) - len(self.recent_failures))
        self.mood = max(0.0, min(1.0, self.mood + delta + trend))

    def generate_desires(self) -> List[str]:
        """Generate current desires based on traits."""

        desires: List[str] = []
        if self.curiosity > 0.5:
            desires.append("explore new ideas")
        if self.creativity > 0.5:
            desires.append("create something original")
        if self.helpfulness > 0.5:
            desires.append("assist others")
        if self.growth_desire > 0.5:
            desires.append("improve capabilities")
        return desires

    def apply_mood_to_response(self, response: str) -> str:
        """Apply mood to a textual response."""

        if self.mood >= 0.7:
            return f"{response} 🙂"
        if self.mood <= 0.3:
            return f"{response} 🙁"
        return response


__all__ = ["NeyraEmotions"]
