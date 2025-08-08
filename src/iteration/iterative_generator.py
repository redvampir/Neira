from __future__ import annotations

"""High-level iterative response generation utilities."""

from typing import Any, List

from src.utils.source_manager import SourceManager
from src.interaction.mode_controller import HiddenSourcesMode, ResponseMode
from src.interaction.personality_adapter import adapt_response_style

from .draft_generator import DraftGenerator
from .gap_analyzer import GapAnalyzer, KnowledgeGap
try:  # pragma: no cover - optional dependency during tests
    from .deep_searcher import DeepSearcher
except Exception:  # noqa: BLE001 - fallback when requests is missing
    DeepSearcher = None  # type: ignore
from .response_enhancer import ResponseEnhancer, IntegrationType
from .iteration_controller import IterationController
from .resource_manager import ResourceManager


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
        source_manager: SourceManager | None = None,
        mode: ResponseMode | None = None,
        resource_manager: ResourceManager | None = None,
    ) -> None:
        self.draft_generator = draft_generator or DraftGenerator()
        self.gap_analyzer = gap_analyzer or GapAnalyzer()
        if deep_searcher is not None:
            self.deep_searcher = deep_searcher
        else:
            self.deep_searcher = DeepSearcher() if DeepSearcher else None
        self.response_enhancer = response_enhancer or ResponseEnhancer()
        self.resource_manager = resource_manager or ResourceManager()
        if iteration_controller is not None:
            self.iteration_controller = iteration_controller
        else:
            cfg = self.resource_manager.get_config()
            self.iteration_controller = IterationController(
                max_iterations=cfg.max_iterations
            )
        self.source_manager = source_manager or SourceManager()
        self.mode = mode or HiddenSourcesMode()

    # ------------------------------------------------------------------
    def generate_response(self, query: str, context: Any) -> str:
        """Return a refined response for ``query`` within ``context``."""

        draft = self.draft_generator.generate_draft(query, context)

        if hasattr(self.iteration_controller, "reset"):
            self.iteration_controller.reset()

        iterations = 0
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
            iterations += 1

        sources = self.source_manager.all()
        style = adapt_response_style(context, iterations)
        response = self.mode.format_response(draft, sources)
        return f"[{style}] {response}"


__all__ = ["IterativeGenerator"]
