from __future__ import annotations

from typing import Callable, Dict, List, Protocol

import difflib


class SuggestionChooser(Protocol):
    """Callable used to select a correction suggestion."""

    def __call__(
        self, error_type: str, suggestions: List[str]
    ) -> str:  # pragma: no cover - interface definition
        ...


class SelfCorrector:
    """Utility class to perform simple self-corrections on text.

    The corrector is built around *handlers* for different error types.
    Each handler receives the current text and returns a list of suggested
    corrections. The first suggestion is applied by default, but a custom
    chooser callback can be supplied to pick an alternative. New handlers
    can be registered to extend functionality.
    """

    # A tiny dictionary of common misspellings used by the default spelling handler
    _COMMON_SPELLING: Dict[str, str] = {
        "teh": "the",
        "recieve": "receive",
        "adress": "address",
    }

    def __init__(self) -> None:
        self._handlers: Dict[str, Callable[[str], List[str]]] = {}
        # Register default handlers
        self.register_handler("spelling", self._handle_spelling)
        self.register_handler("logic", self._handle_logic)
        self.register_handler("characteristic", self._handle_characteristic)

    # ------------------------------------------------------------------
    def register_handler(
        self, error_type: str, handler: Callable[[str], List[str]]
    ) -> None:
        """Register a new handler for a given error type."""

        self._handlers[error_type] = handler

    # ------------------------------------------------------------------
    def correct_errors(
        self, text: str, chooser: SuggestionChooser | None = None
    ) -> tuple[str, Dict[str, List[str]]]:
        """Apply all registered handlers sequentially.

        Parameters
        ----------
        text:
            Source text to correct.
        chooser:
            Optional callback allowing the caller to choose one of the
            suggestions provided by a handler. When omitted, the first
            suggestion is used.

        Returns
        -------
        tuple[str, Dict[str, List[str]]]
            A tuple containing the final corrected text and a mapping of
            error type to the suggestions produced by the corresponding
            handler.
        """

        suggestions_map: Dict[str, List[str]] = {}
        current_text = text
        for error_type, handler in self._handlers.items():
            suggestions = handler(current_text)
            suggestions_map[error_type] = suggestions
            if not suggestions:
                continue
            chosen = suggestions[0]
            if chooser is not None:
                try:
                    choice = chooser(error_type, suggestions)
                    if choice in suggestions:
                        chosen = choice
                except Exception:  # pragma: no cover - defensive; chooser is external
                    pass
            current_text = chosen
        return current_text, suggestions_map

    # ------------------------------------------------------------------
    def _handle_spelling(self, text: str) -> List[str]:
        """Very small spell correction based on common mistakes."""

        words = text.split()
        changed = False
        corrected: List[str] = []
        for word in words:
            lower = word.lower()
            if lower in self._COMMON_SPELLING:
                corrected.append(self._COMMON_SPELLING[lower])
                changed = True
            else:
                # Try a fuzzy match to the tiny dictionary to provide an alternative
                matches = difflib.get_close_matches(
                    lower, self._COMMON_SPELLING.keys(), n=1
                )
                if matches and matches[0] != lower:
                    corrected.append(self._COMMON_SPELLING[matches[0]])
                    changed = True
                else:
                    corrected.append(word)
        if changed:
            return [" ".join(corrected)]
        return []

    # ------------------------------------------------------------------
    def _handle_logic(self, text: str) -> List[str]:
        """Correct simple logical issues like double negatives."""

        corrected = text
        if "not not" in corrected:
            corrected = corrected.replace("not not", "not")
        if corrected != text:
            return [corrected]
        return []

    # ------------------------------------------------------------------
    def _handle_characteristic(self, text: str) -> List[str]:
        """Fix characteristic issues such as repeated words."""

        words = text.split()
        new_words: List[str] = []
        changed = False
        for i, word in enumerate(words):
            if i > 0 and word == words[i - 1]:
                changed = True
                continue
            new_words.append(word)
        if changed:
            return [" ".join(new_words)]
        return []


__all__ = ["SelfCorrector", "SuggestionChooser"]
