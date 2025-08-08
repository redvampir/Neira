from src.interaction.personality_adapter import adapt_response_style
from src.iteration.iterative_generator import IterativeGenerator
from src.iteration.iteration_controller import IterationController


def test_adapt_response_style_basic() -> None:
    assert adapt_response_style({}, 0) == "default_helpful"
    assert adapt_response_style({}, 1) == "confident_but_open"
    assert adapt_response_style({"tone": "curious"}, 1) == "curious_investigator"
    assert adapt_response_style({"tone": "collaborative"}, 3) == "respectful_collaboration"


def test_iterative_generator_includes_style(monkeypatch) -> None:
    class DummyDraftGenerator:
        def generate_draft(self, query, context):
            return "draft ___"

    class DummyGapAnalyzer:
        def analyze(self, draft):
            if "___" in draft:
                from src.iteration.gap_analyzer import KnowledgeGap

                return [KnowledgeGap(claim="info", questions=[], confidence=0.0)]
            return []

    class DummyDeepSearcher:
        def search(self, query, user_id=None, limit=None):
            return [{"content": "resolved"}]

    class DummyEnhancer:
        def enhance(self, draft, search_results, integration, self_correct=True):
            return draft.replace("___", search_results[0]["content"])

    controller = IterationController(max_iterations=3, max_critical_spaces=0)
    generator = IterativeGenerator(
        draft_generator=DummyDraftGenerator(),
        gap_analyzer=DummyGapAnalyzer(),
        deep_searcher=DummyDeepSearcher(),
        response_enhancer=DummyEnhancer(),
        iteration_controller=controller,
    )

    result = generator.generate_response("question", {})
    assert result == "[confident_but_open] draft resolved"
