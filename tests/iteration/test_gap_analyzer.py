from src.iteration import GapAnalyzer, KnowledgeGap
from src.memory import MemoryIndex
from src.analysis.verification_system import VerificationSystem
from src.analysis.uncertainty_manager import UncertaintyManager


def test_detects_unknown_claim():
    analyzer = GapAnalyzer()
    gaps = analyzer.analyze("The sky is green.")
    assert len(gaps) == 1
    gap = gaps[0]
    assert isinstance(gap, KnowledgeGap)
    assert "sky is green" in gap.claim.lower()
    assert gap.disclaimer is not None


def test_verified_claim_produces_no_gap():
    index = MemoryIndex()
    index.set("Earth is round", True, reliability=0.9)
    verifier = VerificationSystem(memory=index, external_checkers=[])
    analyzer = GapAnalyzer(verifier=verifier)
    gaps = analyzer.analyze("Earth is round.")
    assert gaps == []


def test_low_confidence_generates_disclaimer():
    index = MemoryIndex()
    index.set("Water is wet", True, reliability=0.4)
    verifier = VerificationSystem(memory=index, external_checkers=[])
    uncertainty = UncertaintyManager(threshold=0.5)
    analyzer = GapAnalyzer(verifier=verifier, uncertainty=uncertainty)
    gaps = analyzer.analyze("Water is wet.")
    assert len(gaps) == 1
    gap = gaps[0]
    assert gap.confidence == 0.4
    assert gap.disclaimer is not None
