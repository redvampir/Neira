import logging
from src.analysis import grammar_proofreader
from src.analysis.grammar_proofreader import GrammarProofreader


def test_corrects_typo_and_spacing():
    proofreader = GrammarProofreader()
    text = "он пошол домой ,но забыл ключи"
    corrected, changes = proofreader.process(text)
    assert corrected == "он пошёл домой, но забыл ключи"
    assert any(c.get("rule") == "typo" and c.get("before") == "пошол" for c in changes)
    assert any(c.get("rule") == "remove_space_before_comma" for c in changes)


def test_returns_original_when_no_errors():
    proofreader = GrammarProofreader()
    text = "Он пошёл домой, но забыл ключи."
    corrected, changes = proofreader.process(text)
    assert corrected == text
    assert changes == []


def test_warns_when_language_tool_missing(monkeypatch, caplog):
    monkeypatch.setattr(grammar_proofreader, "language_tool_python", None)
    with caplog.at_level(logging.WARNING):
        grammar_proofreader.GrammarProofreader()
    assert any(
        "language_tool_python is not installed" in record.message for record in caplog.records
    )
