from __future__ import annotations

"""Simple claim verification module.

This module provides a tiny framework to verify user claims by consulting
stored memories and optional external checkers.  It is intentionally minimal
and designed primarily for unit testing and demonstration purposes.
"""

from dataclasses import dataclass, field
import re
from typing import Callable, List, Tuple

from src.memory import MemoryIndex


@dataclass
class VerificationResult:
    """Result of a claim verification."""

    claim: str
    verdict: bool | None
    confidence: float
    sources: List[str] = field(default_factory=list)
    clarifying_questions: List[str] = field(default_factory=list)
    disclaimer: str | None = None


class VerificationSystem:
    """Verify claims against memory and external services."""

    def __init__(
        self,
        memory: MemoryIndex | None = None,
        external_checkers: List[Callable[[str], Tuple[bool, float]]] | None = None,
    ) -> None:
        self.memory = memory or MemoryIndex()
        # External checkers are callables returning a tuple of (verdict, confidence)
        self.external_checkers = (
            external_checkers if external_checkers is not None else [self._stub_check]
        )

    # ------------------------------------------------------------------
    def add_external_checker(
        self, checker: Callable[[str], Tuple[bool, float]]
    ) -> None:
        """Register an additional external checker."""

        self.external_checkers.append(checker)

    # ------------------------------------------------------------------
    def verify_claim(self, claim: str) -> VerificationResult:
        """Verify ``claim`` using memory and external checkers."""

        sources: List[str] = []
        verdict: bool | None = None
        confidence = 0.0

        memory_value = self.memory.get(claim)
        if memory_value is not None:
            verdict = bool(memory_value)
            confidence = self.memory.source_reliability.get(claim, 0.0)
            sources.append("memory")

        for checker in self.external_checkers:
            try:
                result, ext_conf = checker(claim)
            except Exception:  # pragma: no cover - defensive, external code
                continue
            sources.append(checker.__name__)
            if verdict is None:
                verdict = result
            elif verdict != result:
                # Conflicting results lower the confidence significantly
                confidence = min(confidence, ext_conf) / 2
                verdict = result
                continue
            confidence = (confidence + ext_conf) / 2 if confidence else ext_conf

        return VerificationResult(claim=claim, verdict=verdict, confidence=confidence, sources=sources)

    # ------------------------------------------------------------------
    def generate_clarifying_questions(self, claim: str, num_questions: int = 2) -> List[str]:
        """Generate clarifying questions for ``claim`` based on simple heuristics."""

        claim_lower = claim.lower()
        questions: List[str] = [f"Что вы подразумеваете под: '{claim}'?"]

        if not re.search(r"\b(кто|кому|кого|чей)\b", claim_lower):
            questions.append("Кто в этом участвует?")

        location_words = r"\b(где|место|локац|город|страна)\b"
        location_pattern = r"\b(?:в|на|из)\s+[A-ZА-ЯЁ]"
        if not re.search(location_words, claim_lower) and not re.search(location_pattern, claim):
            questions.append("Где это происходит?")

        if not (
            re.search(r"\b(когда|дата|время|срок|год|месяц|день|вчера|сегодня|завтра)\b", claim_lower)
            or re.search(r"\d", claim_lower)
        ):
            questions.append("Когда это происходит?")

        if not re.search(r"\b(почему|зачем|причина)\b", claim_lower):
            questions.append("Почему это происходит?")

        if not re.search(r"\b(как|каким образом)\b", claim_lower):
            questions.append("Как это происходит?")

        while len(questions) < num_questions:
            questions.append(f"Есть ли дополнительные детали о '{claim}'?")

        return questions[:num_questions]

    # ------------------------------------------------------------------
    @staticmethod
    def _stub_check(_claim: str) -> Tuple[bool, float]:
        """Default external check stub returning zero confidence."""

        return False, 0.0


__all__ = ["VerificationSystem", "VerificationResult"]

