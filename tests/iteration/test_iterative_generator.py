from src.iteration.iterative_generator import IterativeGenerator
from src.iteration.gap_analyzer import KnowledgeGap
from src.iteration.iteration_controller import IterationController


class DummyDraftGenerator:
    def generate_draft(self, query, context):
        return "draft ___"


class DummyGapAnalyzer:
    def analyze(self, draft):
        if "___" in draft:
            return [KnowledgeGap(claim="info", questions=[], confidence=0.0)]
        return []


class DummyDeepSearcher:
    def __init__(self):
        self.queries = []

    def search(self, query, user_id=None, limit=None):
        self.queries.append(query)
        return [{"content": "resolved"}]


class DummyResponseEnhancer:
    def enhance(self, draft, search_results, integration, self_correct=True):
        return draft.replace("___", search_results[0]["content"])


def test_iterative_generator_resolves_gap_and_stops():
    controller = IterationController(max_iterations=3, max_critical_spaces=0)
    generator = IterativeGenerator(
        draft_generator=DummyDraftGenerator(),
        gap_analyzer=DummyGapAnalyzer(),
        deep_searcher=DummyDeepSearcher(),
        response_enhancer=DummyResponseEnhancer(),
        iteration_controller=controller,
    )

    result = generator.generate_response("question", {})

    assert result == "[confident_but_open] draft resolved"
    assert generator.deep_searcher.queries == ["info"]
