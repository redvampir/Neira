from src.analysis import SelfCorrector


def test_correct_errors_basic() -> None:
    corrector = SelfCorrector()
    text = "teh cat was not not happy happy"
    corrected, info = corrector.correct_errors(text)
    assert corrected == "the cat was not happy"
    assert info["spelling"]
    assert info["logic"]
    assert info["characteristic"]


def test_custom_handler_and_choice() -> None:
    corrector = SelfCorrector()

    def custom_handler(txt: str) -> list[str]:
        return [txt + "1", txt + "2"]

    corrector.register_handler("custom", custom_handler)

    def choose_second(_type: str, suggestions: list[str]) -> str:
        return suggestions[1]

    corrected, info = corrector.correct_errors("start", chooser=choose_second)
    assert corrected == "start2"
    assert info["custom"] == ["start1", "start2"]
