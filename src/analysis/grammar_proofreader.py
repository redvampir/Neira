"""Simple grammar proofreader using language_tool_python when available."""
from __future__ import annotations

import re
from typing import Dict, List, Tuple

from src.core.config import get_logger

from .post_processor import PostProcessor

try:  # pragma: no cover - optional dependency
    import language_tool_python  # type: ignore
except Exception:  # pragma: no cover
    language_tool_python = None  # type: ignore


class GrammarProofreader(PostProcessor):
    """Apply basic grammar and punctuation corrections."""

    def __init__(self, language: str = "ru-RU") -> None:
        self.language = language
        self.tool = None
        if language_tool_python is not None:  # pragma: no cover - optional
            try:
                self.tool = language_tool_python.LanguageTool(language)
            except Exception:
                self.tool = None
        else:
            get_logger(__name__).warning(
                "language_tool_python is not installed; grammar proofreading quality will be reduced"
            )

    def proofread(self, text: str) -> Tuple[str, List[Dict[str, str]]]:
        """Return corrected text and list of applied corrections.

        Each correction is a dictionary describing what change was applied. The
        minimal keys are ``rule`` describing the type of correction and, when
        applicable, ``before``/``after`` with the original and replacement
        fragments.
        """

        if not text:
            return text, []
        corrections: List[Dict[str, str]] = []

        if self.tool is not None:  # pragma: no cover - optional
            matches = self.tool.check(text)
            try:
                corrected = language_tool_python.utils.correct(text, matches)
            except Exception:
                corrected = text
            for m in matches:
                entry: Dict[str, str] = {
                    "rule": m.ruleId or "grammar",
                    "message": m.message,
                }
                if m.replacements:
                    entry["after"] = m.replacements[0]
                corrections.append(entry)
            return corrected, corrections

        corrected = text

        def _record(rule: str, before: str | None = None, after: str | None = None) -> None:
            entry: Dict[str, str] = {"rule": rule}
            if before is not None:
                entry["before"] = before
            if after is not None:
                entry["after"] = after
            corrections.append(entry)

        new = re.sub(r"\s+,", ",", corrected)
        if new != corrected:
            _record("remove_space_before_comma", before=" ,", after=",")
            corrected = new
        new = re.sub(r",(?=\S)", ", ", corrected)
        if new != corrected:
            _record("space_after_comma", before=",", after=", ")
            corrected = new
        new = re.sub(r"\s+\.", ".", corrected)
        if new != corrected:
            _record("remove_space_before_period", before=" .", after=".")
            corrected = new
        replacements = {"пошол": "пошёл", "превет": "привет"}
        for wrong, right in replacements.items():
            pattern = re.compile(wrong, re.IGNORECASE)

            def repl(match: re.Match[str]) -> str:
                _record("typo", before=match.group(0), after=right)
                return right

            new = pattern.sub(repl, corrected)
            corrected = new

        return corrected, corrections

    def process(self, text: str) -> Tuple[str, List[Dict[str, str]]]:
        """Process text according to :class:`PostProcessor` interface."""
        return self.proofread(text)
