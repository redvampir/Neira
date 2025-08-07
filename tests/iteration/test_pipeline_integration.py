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


def test_two_passes_with_grammar_correction(monkeypatch):
    neyra = Neyra()

    draft = "Превет, я пошол домой , все хорошо ."
    expected = "привет, я пошёл домой, все хорошо."

    def fake_process(text: str) -> str:
        neyra.last_draft = draft
        return draft

    monkeypatch.setattr(neyra, "process_command", fake_process)

    gap_calls = []

    def fake_analyze(_draft: str):
        gap_calls.append(_draft)
        return []

    monkeypatch.setattr(neyra.gap_analyzer, "analyze", fake_analyze)

    corrections: list[str] = []
    original_proofread = neyra.grammar_proofreader.proofread

    def fake_proofread(text: str):
        corrected, applied = original_proofread(text)
        corrections.extend(applied)
        return corrected, applied

    monkeypatch.setattr(neyra.grammar_proofreader, "proofread", fake_proofread)

    result = neyra.iterative_response("request")

    assert result == expected
    assert len(corrections) > 0
    assert len(gap_calls) == 1
    assert neyra.iteration_controller._iterations == 2
