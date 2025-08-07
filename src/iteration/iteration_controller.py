from __future__ import annotations

"""Control iterative improvement loops based on quality and limits."""

from dataclasses import dataclass

from src.core.neyra_config import NeyraPersonality

from .strategy_manager import AdaptiveIterationManager


@dataclass
class IterationController:
    """Manage iteration process with simple quality checks.

    Parameters
    ----------
    strategy:
        Name of the iteration strategy preset managed by
        :class:`AdaptiveIterationManager`.  Defaults to ``"standard"``.
    max_iterations:
        Maximum number of additional iterations allowed. When ``None`` the
        value from the selected ``strategy`` is used.
    max_critical_spaces:
        Threshold for unresolved placeholders (``"___"``) allowed in a response
        before stopping the loop. When ``None`` the value from ``strategy`` is
        applied.
    """

    strategy: str = "standard"
    max_iterations: int | None = None
    max_critical_spaces: int | None = None
    personality: NeyraPersonality | None = None
    emotional_state: str = "neutral"
    _iterations: int = 0

    def __post_init__(self) -> None:
        manager = AdaptiveIterationManager(self.strategy)
        if self.max_iterations is None:
            self.max_iterations = manager.max_iterations
        if self.max_critical_spaces is None:
            self.max_critical_spaces = manager.max_critical_spaces

    # ------------------------------------------------------------------
    def _priority_multiplier(self) -> float:
        """Return weighting factor based on personality and emotion."""

        factor = 1.0
        if self.personality:
            # Combine relevant personality traits
            factor *= (
                self.personality.curiosity_level
                + self.personality.attention_to_detail
            ) / 2
        mood = {
            "спокойная": 0.9,
            "любопытная": 1.0,
            "взволнованная": 1.1,
        }
        factor *= mood.get(self.emotional_state, 1.0)
        return factor

    def assess_quality(self, text: str) -> float:
        """Return weighted count of critical placeholders in ``text``.

        A *critical space* is represented by the sequence ``"___"``. Each
        occurrence signals missing or uncertain information that should be
        resolved before finalising the response.  The count is weighted by
        :class:`NeyraPersonality` and current ``emotional_state`` to reflect
        how much attention should be given to these gaps.
        """

        return text.count("___") * self._priority_multiplier()

    # ------------------------------------------------------------------
    def should_iterate(self, text: str) -> bool:
        """Return ``True`` if another refinement iteration is required.

        The decision is based on two criteria:

        * the response still contains more than ``max_critical_spaces``
          placeholders, indicating low quality;
        * the number of iterations performed so far is less than
          ``max_iterations``.
        """

        if self._iterations >= self.max_iterations:
            return False

        gaps = self.assess_quality(text)
        if gaps <= self.max_critical_spaces:
            return False

        self._iterations += 1
        return True


__all__ = ["IterationController"]
