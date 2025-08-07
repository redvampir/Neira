"""Simple grammar proofreader using language_tool_python when available."""
from __future__ import annotations

import re
from typing import List, Tuple

try:  # pragma: no cover - optional dependency
    import language_tool_python  # type: ignore
except Exception:  # pragma: no cover
    language_tool_python = None  # type: ignore


class GrammarProofreader:
    """Apply basic grammar and punctuation corrections."""

    def __init__(self, language: str = "ru-RU") -> None:
        self.language = language
        self.tool = None
        if language_tool_python is not None:  # pragma: no cover - optional
            try:
                self.tool = language_tool_python.LanguageTool(language)
            except Exception:
                self.tool = None

    def proofread(self, text: str) -> Tuple[str, List[str]]:
        """Return corrected text and list of applied corrections."""
        if not text:
            return text, []
        corrections: List[str] = []

        if self.tool is not None:  # pragma: no cover - optional
            matches = self.tool.check(text)
            try:
                corrected = language_tool_python.utils.correct(text, matches)
            except Exception:
                corrected = text
            for m in matches:
                if m.replacements:
                    corrections.append(f"{m.ruleId}:{m.replacements[0]}")
                else:
                    corrections.append(m.message)
            return corrected, corrections

        corrected = text
        new = re.sub(r"\s+,", ",", corrected)
        if new != corrected:
            corrections.append("remove_space_before_comma")
            corrected = new
        new = re.sub(r",(?=\S)", ", ", corrected)
        if new != corrected:
            corrections.append("space_after_comma")
            corrected = new
        new = re.sub(r"\s+\.", ".", corrected)
        if new != corrected:
            corrections.append("remove_space_before_period")
            corrected = new
        replacements = {"пошол": "пошёл", "превет": "привет"}
        for wrong, right in replacements.items():
            pattern = re.compile(wrong, re.IGNORECASE)
            new = pattern.sub(right, corrected)
            if new != corrected:
                corrections.append(f"{wrong}->{right}")
                corrected = new
        return corrected, corrections
