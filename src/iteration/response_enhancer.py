from __future__ import annotations

"""Utilities to enhance draft responses with external information."""

from enum import Enum
from typing import Any, Dict, List

from src.analysis import SelfCorrector


class IntegrationType(Enum):
    """Supported strategies for integrating search results."""

    CRITICAL_CORRECTION = "CRITICAL_CORRECTION"
    IMPORTANT_ADDITION = "IMPORTANT_ADDITION"
    CONTEXT_ENRICHMENT = "CONTEXT_ENRICHMENT"


class ResponseEnhancer:
    """Combine a draft with search results to produce an improved response.

    Parameters
    ----------
    corrector:
        Optional :class:`~src.analysis.self_corrector.SelfCorrector` instance. When
        provided it will be used to run a light self-correction pass on the
        enhanced text before returning it to the caller.
    """

    def __init__(self, corrector: SelfCorrector | None = None) -> None:
        self.corrector = corrector or SelfCorrector()

    # ------------------------------------------------------------------
    def enhance(
        self,
        draft: str,
        search_results: List[Dict[str, Any]],
        integration: IntegrationType,
        *,
        self_correct: bool = True,
    ) -> str:
        """Return an improved version of ``draft``.

        The function applies the strategy specified by ``integration`` to mix the
        search results into the draft:

        ``CRITICAL_CORRECTION``
            Replace the draft entirely with the highest priority search result.
        ``IMPORTANT_ADDITION``
            Append all search result snippets to the draft separated by newlines.
        ``CONTEXT_ENRICHMENT``
            Prepend the snippets before the draft to provide additional context.

        When ``self_correct`` is ``True`` the resulting text is passed through the
        :class:`SelfCorrector` to fix simple issues like typos.
        """

        snippets = [str(r.get("content", "")) for r in search_results if r.get("content")]
        text = draft

        if integration == IntegrationType.CRITICAL_CORRECTION:
            if snippets:
                text = snippets[0]
        elif integration == IntegrationType.IMPORTANT_ADDITION:
            if snippets:
                addition = "\n".join(snippets)
                text = f"{text}\n{addition}" if text else addition
        elif integration == IntegrationType.CONTEXT_ENRICHMENT:
            if snippets:
                context = "\n".join(snippets)
                text = f"{context}\n{text}" if text else context
        else:  # pragma: no cover - defensive branch
            raise ValueError(f"Unknown integration type: {integration}")

        if self_correct:
            text, _ = self.corrector.correct_errors(text)
        return text


__all__ = ["ResponseEnhancer", "IntegrationType"]
