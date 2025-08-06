from __future__ import annotations

"""Simple uncertainty management for verification results."""

from typing import List, Tuple

from .verification_system import VerificationResult


class UncertaintyManager:
    """Track and handle low-confidence verification results."""

    def __init__(self, threshold: float = 0.5) -> None:
        self.threshold = threshold
        self._records: List[Tuple[bool, float]] = []

    # ------------------------------------------------------------------
    def handle(self, result: VerificationResult) -> VerificationResult:
        """Attach disclaimer if confidence is below the threshold."""

        if result.confidence < self.threshold:
            result.disclaimer = (
                f"⚠️ Моя уверенность в этом ответе низка ({result.confidence:.2f})."
            )
        return result

    # ------------------------------------------------------------------
    def add_record(self, truth: bool, confidence: float) -> None:
        """Store an observation for calibration metrics."""

        self._records.append((truth, confidence))

    # ------------------------------------------------------------------
    def accuracy(self) -> float:
        """Compute simple accuracy against the threshold."""

        if not self._records:
            return 0.0
        correct = sum(1 for truth, conf in self._records if (conf >= self.threshold) == truth)
        return correct / len(self._records)

    # ------------------------------------------------------------------
    def calibration_error(self) -> float:
        """Calculate a basic calibration error (Brier score)."""

        if not self._records:
            return 0.0
        return sum((conf - (1.0 if truth else 0.0)) ** 2 for truth, conf in self._records) / len(
            self._records
        )


__all__ = ["UncertaintyManager"]
