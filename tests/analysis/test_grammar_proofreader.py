from src.analysis.grammar_proofreader import GrammarProofreader


def test_corrects_typo_and_spacing():
    proofreader = GrammarProofreader()
    text = "он пошол домой ,но забыл ключи"
    corrected, changes = proofreader.proofread(text)
    assert corrected == "он пошёл домой, но забыл ключи"
    assert "пошол->пошёл" in changes
    assert "remove_space_before_comma" in changes


def test_returns_original_when_no_errors():
    proofreader = GrammarProofreader()
    text = "Он пошёл домой, но забыл ключи."
    corrected, changes = proofreader.proofread(text)
    assert corrected == text
    assert changes == []
