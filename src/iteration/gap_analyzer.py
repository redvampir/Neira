from __future__ import annotations

"""Detect missing knowledge in draft responses."""

from dataclasses import dataclass
import re
from typing import List

from src.analysis.verification_system import VerificationSystem, VerificationResult
from src.analysis.uncertainty_manager import UncertaintyManager
from src.analysis.timeline_checker import TimelineChecker


@dataclass
class KnowledgeGap:
    """Information missing or uncertain in a draft."""

    claim: str
    questions: List[str]
    confidence: float
    disclaimer: str | None = None
    gap_type: str = "knowledge_gap"


class GapAnalyzer:
    """Analyze text and highlight claims with low confidence."""

    def __init__(
        self,
        verifier: VerificationSystem | None = None,
        uncertainty: UncertaintyManager | None = None,
        timeline_checker: TimelineChecker | None = None,
    ) -> None:
        self.verifier = verifier or VerificationSystem()
        self.uncertainty = uncertainty or UncertaintyManager()
        self.timeline_checker = timeline_checker or TimelineChecker()

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
        timeline_conflicts = self.timeline_checker.check()
        for conflict in timeline_conflicts:
            gaps.append(
                KnowledgeGap(
                    claim=conflict,
                    questions=[],
                    confidence=0.0,
                    gap_type="timeline_conflict",
                )
            )
        return gaps


__all__ = ["GapAnalyzer", "KnowledgeGap"]
