from __future__ import annotations

"""High-level iterative response generation utilities."""

from typing import Any, List
import time

from src.monitoring.metrics_monitor import MetricsMonitor
from src.monitoring.iteration_logger import IterationLogger

from src.utils.source_manager import SourceManager
from src.interaction.mode_controller import HiddenSourcesMode, ResponseMode
from src.interaction.personality_adapter import (
    PersonalityAdapter,
    adapt_response_style,
)
from src.quality import GrammarIssue
from src.plugins import PluginManager

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
        metrics_monitor: MetricsMonitor | None = None,
        iteration_logger: IterationLogger | None = None,
        plugin_manager: PluginManager | None = None,
        personality_adapter: PersonalityAdapter | None = None,
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
        self.metrics_monitor = metrics_monitor
        self.iteration_logger = iteration_logger
        self.plugin_manager = plugin_manager or PluginManager()
        self.personality_adapter = personality_adapter or PersonalityAdapter()

    # ------------------------------------------------------------------
    def generate_response(self, query: str, context: Any) -> str:
        """Return a refined response for ``query`` within ``context``."""

        start_time = time.perf_counter()
        draft = self.draft_generator.generate_draft(query, context)
        if self.plugin_manager:
            self.plugin_manager.on_draft(draft, context)

        if hasattr(self.iteration_controller, "reset"):
            self.iteration_controller.reset()

        iterations = 0
        rules_refs: List[GrammarIssue] = []
        while self.iteration_controller.should_iterate(draft):
            gaps: List[KnowledgeGap] = self.gap_analyzer.analyze(draft)
            if self.plugin_manager:
                self.plugin_manager.on_gap_analysis(draft, gaps)

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

            for result in search_results:
                self.source_manager.register(
                    result["content"],
                    result["reference"],
                    result["priority"],
                )

            self.source_manager.limit_sources(context)

            result = self.response_enhancer.enhance(
                draft,
                search_results,
                IntegrationType.IMPORTANT_ADDITION,
            )
            if isinstance(result, dict):
                draft = result.get("text", "")
                rules_refs = result.get("rules_refs", [])
            else:
                draft = result
                rules_refs = []
            iterations += 1

            resource_metrics = None
            if self.resource_manager:
                cpu, memory = self.resource_manager.update_usage()
                resource_metrics = {"cpu": cpu, "memory": memory}

            if self.metrics_monitor and resource_metrics is not None:
                self.metrics_monitor.log_performance_metrics(**resource_metrics)

            if self.iteration_logger:
                self.iteration_logger.log_iteration(
                    iterations,
                    draft,
                    gaps,
                    search_results,
                    result,
                    resource_metrics=resource_metrics,
                )

        sources = self.source_manager.all()
        style = adapt_response_style(context, iterations)
        formatted_rules = self.personality_adapter.format_rules(rules_refs)
        response = self.mode.format_response(draft, sources, formatted_rules)
        if self.plugin_manager:
            self.plugin_manager.on_finalize(response)

        if self.metrics_monitor:
            duration = time.perf_counter() - start_time
            final_quality = self.iteration_controller.assess_quality(draft)
            self.metrics_monitor.log_performance_metrics(
                duration=duration,
                iterations=iterations,
                num_sources=len(sources),
            )
            self.metrics_monitor.log_quality_metrics(final_quality=final_quality)

        return f"[{style}] {response}"


__all__ = ["IterativeGenerator"]
