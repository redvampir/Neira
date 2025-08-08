from src.quality import GrammarRuleChecker


def test_detects_rules() -> None:
    checker = GrammarRuleChecker()
    text = "Я живу в г Москва  сейчас"
    issues = checker.check(text)
    ids = {issue.rule_id for issue in issues}
    assert "point_after_abbrev" in ids
    assert "double_space" in ids
