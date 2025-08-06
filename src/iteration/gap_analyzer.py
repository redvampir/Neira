from __future__ import annotations

"""Detect missing knowledge in draft responses."""

from dataclasses import dataclass
import re
from typing import List

from src.analysis.verification_system import VerificationSystem, VerificationResult
from src.analysis.uncertainty_manager import UncertaintyManager


@dataclass
class KnowledgeGap:
    """Information missing or uncertain in a draft."""

    claim: str
    questions: List[str]
    confidence: float
    disclaimer: str | None = None


class GapAnalyzer:
    """Analyze text and highlight claims with low confidence."""

    def __init__(
        self,
        verifier: VerificationSystem | None = None,
        uncertainty: UncertaintyManager | None = None,
    ) -> None:
        self.verifier = verifier or VerificationSystem()
        self.uncertainty = uncertainty or UncertaintyManager()

    # ------------------------------------------------------------------
    def analyze(self, draft: str) -> List[KnowledgeGap]:
        """Return knowledge gaps detected in ``draft`` text."""

        gaps: List[KnowledgeGap] = []
        sentences = [s.strip() for s in re.split(r"[.!?\n]+", draft) if s.strip()]
        for claim in sentences:
            result: VerificationResult = self.verifier.verify_claim(claim)
            result = self.uncertainty.handle(result)
            if result.verdict is False or result.confidence < self.uncertainty.threshold:
                questions = self.verifier.generate_clarifying_questions(claim)
                gaps.append(
                    KnowledgeGap(
                        claim=claim,
                        questions=questions,
                        confidence=result.confidence,
                        disclaimer=result.disclaimer,
                    )
                )
        return gaps


__all__ = ["GapAnalyzer", "KnowledgeGap"]
