from src.interaction.mode_controller import VisibleSourcesMode
from src.interaction.personality_adapter import PersonalityAdapter, adapt_response_style
from src.iteration.iterative_generator import IterativeGenerator
from src.iteration.iteration_controller import IterationController
from src.quality import GrammarRuleChecker


def test_adapt_response_style_basic() -> None:
    assert adapt_response_style({}, 0) == "default_helpful"
    assert adapt_response_style({}, 1) == "confident_but_open"
    assert adapt_response_style({"tone": "curious"}, 1) == "curious_investigator"
    assert adapt_response_style({"tone": "collaborative"}, 3) == "respectful_collaboration"


def test_iterative_generator_includes_style_and_rules(monkeypatch) -> None:
    class DummyDraftGenerator:
        def generate_draft(self, query, context):
            return "draft  ___"  # double space triggers rule

    class DummyGapAnalyzer:
        def analyze(self, draft):
            if "___" in draft:
                from src.iteration.gap_analyzer import KnowledgeGap

                return [KnowledgeGap(claim="info", questions=[], confidence=0.0)]
            return []

    class DummyDeepSearcher:
        def search(self, query, user_id=None, limit=None):
            return [
                {"content": "resolved", "reference": "ref", "priority": 0.5}
            ]

    class DummyEnhancer:
        def __init__(self) -> None:
            self.checker = GrammarRuleChecker()

        def enhance(self, draft, search_results, integration, self_correct=True):
            text = draft.replace("___", search_results[0]["content"])
            issues = self.checker.check(text)
            return {"text": text, "rules_refs": issues}

    controller = IterationController(max_iterations=3, max_critical_spaces=0)
    adapter = PersonalityAdapter(explain_rules=True)
    generator = IterativeGenerator(
        draft_generator=DummyDraftGenerator(),
        gap_analyzer=DummyGapAnalyzer(),
        deep_searcher=DummyDeepSearcher(),
        response_enhancer=DummyEnhancer(),
        iteration_controller=controller,
        mode=VisibleSourcesMode(),
        personality_adapter=adapter,
    )

    result, _ = generator.generate_response("question", {})
    assert "[confident_but_open]" in result
    assert "draft  resolved" in result
    assert "см. §2" in result
    assert "замените несколько пробелов одним" in result
