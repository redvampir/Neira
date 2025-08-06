from src.analysis import VerificationSystem, verify_fact
from src.memory import MemoryIndex


def _dummy_check(_claim: str) -> tuple[bool, float]:
    return True, 0.8


def test_verify_claim_with_memory_and_external() -> None:
    memory = MemoryIndex()
    memory.set("the sky is blue", True, reliability=0.9)
    verifier = VerificationSystem(memory=memory, external_checkers=[_dummy_check])
    result = verifier.verify_claim("the sky is blue")
    assert result.verdict is True
    assert result.confidence >= 0.8
    assert "memory" in result.sources
    assert any("_dummy_check" in s for s in result.sources)


def test_generate_clarifying_questions() -> None:
    verifier = VerificationSystem()
    questions = verifier.generate_clarifying_questions("the sky is blue")
    assert questions
    assert "the sky is blue" in questions[0]


def test_verify_fact_success() -> None:
    def fake_search(query: str, limit: int):
        return [{"snippet": f"Some text {query} end"}]

    assert verify_fact("the sky is blue", search_func=fake_search)


def test_verify_fact_failure() -> None:
    def fake_search(_query: str, limit: int):
        return [{"snippet": "unrelated"}]

    assert not verify_fact("the sky is blue", search_func=fake_search)
