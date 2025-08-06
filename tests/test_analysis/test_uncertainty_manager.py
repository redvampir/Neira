from src.analysis import VerificationResult, UncertaintyManager, VerificationSystem
import pytest


def test_disclaimer_added_on_low_confidence() -> None:
    manager = UncertaintyManager(threshold=0.7)
    vs = VerificationSystem()
    result = VerificationResult(claim="sky is green", verdict=False, confidence=0.3)
    result = manager.handle(result)
    if result.confidence < manager.threshold:
        result.clarifying_questions = vs.generate_clarifying_questions(result.claim)
    assert result.disclaimer is not None
    assert result.clarifying_questions


def test_calibration_metrics() -> None:
    manager = UncertaintyManager(threshold=0.5)
    manager.add_record(True, 0.9)
    manager.add_record(False, 0.2)
    assert manager.accuracy() == pytest.approx(1.0)
    assert manager.calibration_error() == pytest.approx((0.1**2 + 0.2**2) / 2)
