from __future__ import annotations

"""High-level iterative response generation utilities."""

from typing import Any, List

from .draft_generator import DraftGenerator
from .gap_analyzer import GapAnalyzer, KnowledgeGap
try:  # pragma: no cover - optional dependency during tests
    from .deep_searcher import DeepSearcher
except Exception:  # noqa: BLE001 - fallback when requests is missing
    DeepSearcher = None  # type: ignore
from .response_enhancer import ResponseEnhancer, IntegrationType
from .iteration_controller import IterationController


class IterativeGenerator:
    """Generate responses through repeated refinement cycles.

    The generator coordinates a group of collaborative components:

    * ``DraftGenerator`` produces an initial draft for a query.
    * ``GapAnalyzer`` inspects the draft for missing or uncertain information.
    * ``DeepSearcher`` retrieves additional information for each detected gap.
    * ``ResponseEnhancer`` integrates search results back into the draft.
    * ``IterationController`` decides whether another iteration is required.
    """

    def __init__(
        self,
        draft_generator: DraftGenerator | None = None,
        gap_analyzer: GapAnalyzer | None = None,
        deep_searcher: DeepSearcher | None = None,
        response_enhancer: ResponseEnhancer | None = None,
        iteration_controller: IterationController | None = None,
    ) -> None:
        self.draft_generator = draft_generator or DraftGenerator()
        self.gap_analyzer = gap_analyzer or GapAnalyzer()
        if deep_searcher is not None:
            self.deep_searcher = deep_searcher
        else:
            self.deep_searcher = DeepSearcher() if DeepSearcher else None
        self.response_enhancer = response_enhancer or ResponseEnhancer()
        self.iteration_controller = iteration_controller or IterationController()

    # ------------------------------------------------------------------
    def generate_response(self, query: str, context: Any) -> str:
        """Return a refined response for ``query`` within ``context``."""

        draft = self.draft_generator.generate_draft(query, context)

        if hasattr(self.iteration_controller, "reset"):
            self.iteration_controller.reset()

        while self.iteration_controller.should_iterate(draft):
            gaps: List[KnowledgeGap] = self.gap_analyzer.analyze(draft)

            search_results = []
            if self.deep_searcher is not None:
                for gap in gaps:
                    try:
                        search_results.extend(self.deep_searcher.search(gap.claim))
                        for question in getattr(gap, "questions", []):
                            search_results.extend(
                                self.deep_searcher.search(question)
                            )
                    except Exception:
                        continue

            draft = self.response_enhancer.enhance(
                draft,
                search_results,
                IntegrationType.IMPORTANT_ADDITION,
            )

        return draft


__all__ = ["IterativeGenerator"]
