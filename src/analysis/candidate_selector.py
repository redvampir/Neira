from __future__ import annotations

"""Select the best response candidate based on simple heuristics."""

from typing import List

from .verification_system import VerificationSystem


class CandidateSelector:
    """Score and choose among multiple response candidates.

    The selection combines similarity with stored memory entries and the
    confidence provided by the :class:`VerificationSystem`.
    """

    def __init__(self, verifier: VerificationSystem) -> None:
        self.verifier = verifier

    # ------------------------------------------------------------------
    def _memory_score(self, text: str) -> float:
        """Return ``1.0`` if ``text`` resembles known memory, else ``0.0``."""

        return 1.0 if self.verifier.memory.similar(text, 1) else 0.0

    # ------------------------------------------------------------------
    def score(self, text: str) -> float:
        """Compute overall score for ``text``."""

        verification = self.verifier.verify_claim(text)
        memory_score = self._memory_score(text)
        return (memory_score + verification.confidence) / 2

    # ------------------------------------------------------------------
    def select_best(self, candidates: List[str]) -> str:
        """Return the candidate with the highest :meth:`score`."""

        if not candidates:
            return ""
        return max(candidates, key=self.score)
