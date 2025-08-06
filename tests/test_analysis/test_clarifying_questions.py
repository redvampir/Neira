from src.analysis import VerificationSystem
from src.core.neyra_brain import Neyra


def test_generate_clarifying_questions() -> None:
    verifier = VerificationSystem()
    questions = verifier.generate_clarifying_questions("Построить дом", num_questions=5)
    assert any("Где" in q for q in questions)
    assert any("Когда" in q for q in questions)

    specific = verifier.generate_clarifying_questions(
        "Построить дом в Москве в 2024 году", num_questions=5
    )
    assert all("Где" not in q for q in specific)
    assert all("Когда" not in q for q in specific)


def test_neyra_generates_questions_on_low_confidence() -> None:
    neyra = Neyra()
    result = neyra.verify_claim("Небо зелёное")
    assert result.clarifying_questions

