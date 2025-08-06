from __future__ import annotations

"""Control iterative improvement loops based on quality and limits."""

from dataclasses import dataclass


@dataclass
class IterationController:
    """Manage iteration process with simple quality checks.

    Parameters
    ----------
    max_iterations:
        Maximum number of additional iterations allowed. The controller assumes
        there is already an initial response (iteration 0) and counts only
        further refinement steps.
    max_critical_spaces:
        Threshold for unresolved placeholders (``"___"``) allowed in a response
        before stopping the loop.
    """

    max_iterations: int = 3
    max_critical_spaces: int = 0
    _iterations: int = 0

    # ------------------------------------------------------------------
    def assess_quality(self, text: str) -> int:
        """Return the number of critical placeholders remaining in ``text``.

        A *critical space* is represented by the sequence ``"___"``. Each
        occurrence signals missing or uncertain information that should be
        resolved before finalising the response.
        """

        return text.count("___")

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
