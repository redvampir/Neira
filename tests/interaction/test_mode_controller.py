from __future__ import annotations

from src.interaction.mode_controller import (
    HiddenSourcesMode,
    LightweightMode,
    VisibleSourcesMode,
)
from src.utils.source_manager import ManagedSource, SourceManager
from src.iteration.iterative_generator import IterativeGenerator
from src.iteration.iteration_controller import IterationController


SOURCES = [
    ManagedSource(summary="a", path="http://a", reliability=1.0),
    ManagedSource(summary="b", path="http://b", reliability=0.5),
]


def test_hidden_mode_returns_content_only() -> None:
    mode = HiddenSourcesMode()
    content = mode.format_response("answer", SOURCES)
    assert content == "answer"


def test_visible_mode_appends_summaries_and_paths() -> None:
    mode = VisibleSourcesMode()
    content = mode.format_response("answer", SOURCES)
    expected = (
        "answer\n\nSources:\n"
        "[1] a (http://a)\n"
        "[2] b (http://b)"
    )
    assert content == expected


def test_lightweight_mode_appends_only_paths() -> None:
    mode = LightweightMode()
    content = mode.format_response("answer", SOURCES)
    expected = (
        "answer\n\nSources:\n"
        "[1] http://a\n"
        "[2] http://b"
    )
    assert content == expected


def test_iterative_generator_uses_provided_mode() -> None:
    manager = SourceManager()
    manager.register("a", "http://a", 1.0)

    class DummyDraft:
        def generate_draft(self, query, context):
            return "answer"

    class DummyGapAnalyzer:
        def analyze(self, draft):
            return []

    class DummyEnhancer:
        def enhance(self, draft, search_results, integration, self_correct=True):
            return draft

    controller = IterationController(max_iterations=1, max_critical_spaces=0)
    generator = IterativeGenerator(
        draft_generator=DummyDraft(),
        gap_analyzer=DummyGapAnalyzer(),
        response_enhancer=DummyEnhancer(),
        iteration_controller=controller,
        source_manager=manager,
        mode=VisibleSourcesMode(),
    )

    result = generator.generate_response("q", {})
    assert result == "[default_helpful] answer\n\nSources:\n[1] a (http://a)"
