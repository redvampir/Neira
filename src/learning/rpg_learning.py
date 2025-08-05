from __future__ import annotations

"""Simple adaptive learning helpers for game masters."""

from collections import Counter
from typing import Dict, List, Mapping, Sequence


class RPGLearningSystem:
    """Lightweight system that learns from game sessions.

    The class keeps track of the master's skills, personality adjustments
    and notable successful scenarios. It exposes small utility methods
    that operate on basic Python data structures so they can be easily
    tested and integrated.
    """

    def __init__(self) -> None:
        self.skill_ratings: Dict[str, int] = {}
        self.personality_traits: Dict[str, float] = {}
        self.successful_scenarios: List[str] = []

    def analyze_player_reactions(self, reactions: Sequence[str]) -> Dict[str, int]:
        """Count occurrences of each reaction in ``reactions``.

        Parameters
        ----------
        reactions:
            Collection of player reaction labels.

        Returns
        -------
        dict
            Mapping of reaction label to number of times it appeared.
        """

        return dict(Counter(reactions))

    def improve_master_skills(self, feedback: Mapping[str, int]) -> Dict[str, int]:
        """Update internal skill ratings using ``feedback`` increments."""

        for skill, delta in feedback.items():
            self.skill_ratings[skill] = self.skill_ratings.get(skill, 0) + int(delta)
        return dict(self.skill_ratings)

    def adapt_personality_behaviors(
        self, adjustments: Mapping[str, float]
    ) -> Dict[str, float]:
        """Apply ``adjustments`` to personality traits, clamped to [-1.0, 1.0]."""

        for trait, delta in adjustments.items():
            value = self.personality_traits.get(trait, 0.0) + float(delta)
            self.personality_traits[trait] = max(-1.0, min(1.0, value))
        return dict(self.personality_traits)

    def learn_from_successful_scenarios(self, scenarios: Sequence[str]) -> List[str]:
        """Record unique ``scenarios`` that yielded positive results."""

        for scenario in scenarios:
            if scenario not in self.successful_scenarios:
                self.successful_scenarios.append(scenario)
        return list(self.successful_scenarios)
