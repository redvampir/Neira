from __future__ import annotations

"""Simple rule-based grammar checker."""

from dataclasses import dataclass
import re
from typing import Dict, List


@dataclass
class GrammarIssue:
    """Represents a detected grammar issue."""

    rule_id: str
    snippet: str
    suggestion: str
    ref: str


RULES: Dict[str, Dict[str, object]] = {
    "point_after_abbrev": {
        "pattern": re.compile(r"\bг(?!\.)\s+[A-ZА-Я][\w-]*"),
        "suggestion": "используйте 'г.' после сокращения",
        "ref": "§1",
    },
    "double_space": {
        "pattern": re.compile(r" {2,}"),
        "suggestion": "замените несколько пробелов одним",
        "ref": "§2",
    },
}


class GrammarRuleChecker:
    """Check text against a fixed set of grammar rules."""

    def __init__(self, rules: Dict[str, Dict[str, object]] | None = None) -> None:
        self.rules = rules or RULES

    def check(self, text: str) -> List[GrammarIssue]:
        """Return list of grammar issues found in ``text``."""

        issues: List[GrammarIssue] = []
        for rule_id, data in self.rules.items():
            pattern: re.Pattern[str] = data["pattern"]  # type: ignore[assignment]
            suggestion = str(data.get("suggestion", ""))
            ref = str(data.get("ref", ""))
            for match in pattern.finditer(text):
                snippet = match.group(0)
                issues.append(GrammarIssue(rule_id, snippet, suggestion, ref))
        return issues


__all__ = ["GrammarRuleChecker", "GrammarIssue", "RULES"]
