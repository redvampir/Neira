import importlib.util
import pathlib
import time

spec = importlib.util.spec_from_file_location(
    "verification_system", pathlib.Path("src/analysis/verification_system.py")
)
verification_system = importlib.util.module_from_spec(spec)
spec.loader.exec_module(verification_system)  # type: ignore

VerificationSystem = verification_system.VerificationSystem
verify_claim = verification_system.verify_claim
verify_fact = verification_system.verify_fact


def test_verify_claim_basic() -> None:
    verifier = VerificationSystem()
    context = ["the sky is blue", "grass is green"]
    result = verifier.verify_claim("the sky is blue", context)
    assert result.verdict is True
    assert result.confidence == 1.0


def test_verify_claim_performance() -> None:
    context = ["foo"] * 10000
    start = time.time()
    result = verify_claim("foo", context)
    duration = time.time() - start
    assert result.verdict is True
    assert duration < 0.5


def test_verify_fact_success() -> None:
    def fake_search(query: str, limit: int):
        return [{"snippet": f"Some text {query} end"}]

    assert verify_fact("the sky is blue", search_func=fake_search)


def test_verify_fact_failure() -> None:
    def fake_search(_query: str, limit: int):
        return [{"snippet": "unrelated"}]

    assert not verify_fact("the sky is blue", search_func=fake_search)
