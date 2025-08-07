from src.core.neyra_brain import Neyra
from src.iteration import KnowledgeGap


def test_iteration_pipeline_order(monkeypatch):
    neyra = Neyra()
    calls = []

    def fake_generate(query, hot):
        calls.append("draft")
        return "draft"

    monkeypatch.setattr(neyra.draft_generator, "generate_draft", fake_generate)

    def fake_process(text):
        neyra.last_draft = neyra.draft_generator.generate_draft(text, None)
        return "initial"

    monkeypatch.setattr(neyra, "process_command", fake_process)

    def fake_analyze(draft):
        calls.append("gap")
        return [KnowledgeGap(claim=draft, questions=[], confidence=0.0)]

    monkeypatch.setattr(neyra.gap_analyzer, "analyze", fake_analyze)

    def fake_search(query, user_id=None, limit=5):
        calls.append("search")
        return [{"content": "info"}]

    monkeypatch.setattr(neyra.deep_searcher, "search", fake_search)

    def fake_enhance(text, results, integration, self_correct=True):
        calls.append("enhance")
        return text + " info"

    monkeypatch.setattr(neyra.response_enhancer, "enhance", fake_enhance)

    def fake_iterate(text):
        calls.append("iterate")
        return False

    monkeypatch.setattr(neyra.iteration_controller, "should_iterate", fake_iterate)

    result = neyra.iterative_response("query")
    assert result == "initial info"
    assert calls == ["draft", "gap", "search", "enhance", "iterate"]
