from __future__ import annotations

"""Adapt response tone based on context and iteration count."""

from typing import Any, Iterable, List

from src.quality import GrammarIssue

# Predefined response styles.
STYLES: dict[str, str] = {
    "confident_but_open": "I am confident in this information, but open to feedback.",
    "curious_investigator": "Let's explore this further with curiosity.",
    "respectful_collaboration": "Working together respectfully for the best answer.",
    "default_helpful": "Here's what I found.",
}


def adapt_response_style(context: Any, iteration_count: int) -> str:
    """Return a style label for a response.

    Parameters
    ----------
    context:
        Arbitrary context data that may include a ``tone`` hint.
    iteration_count:
        Number of refinement iterations performed during generation.
    """

    tone = getattr(context, "get", lambda key, default=None: default)("tone", None)

    if tone == "curious":
        return "curious_investigator"
    if tone == "collaborative":
        return "respectful_collaboration"
    if iteration_count > 0:
        return "confident_but_open"
    return "default_helpful"


class PersonalityAdapter:
    """Format grammar rule references for responses.

    Parameters
    ----------
    explain_rules:
        When ``True``, include rule suggestions from
        :class:`~src.quality.grammar_rule_checker.GrammarRuleChecker` in the
        formatted references.
    """

    def __init__(self, *, explain_rules: bool = False) -> None:
        self.explain_rules = explain_rules

    def format_rules(self, issues: Iterable[GrammarIssue]) -> List[str]:
        """Return formatted references for ``issues``.

        Each rule is rendered as ``"см. правило §<rule_id>"``. If
        :attr:`explain_rules` is ``True`` and a suggestion is available, it is
        appended after the rule identifier.
        """

        refs: List[str] = []
        for issue in issues:
            ref = f"см. правило §{issue.rule_id}"
            if self.explain_rules and issue.suggestion:
                ref = f"{ref}: {issue.suggestion}"
            refs.append(ref)
        return refs


__all__ = ["adapt_response_style", "STYLES", "PersonalityAdapter"]
