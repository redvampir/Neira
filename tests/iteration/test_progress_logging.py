from src.core.neyra_brain import Neyra
from src.iteration import KnowledgeGap


def test_progress_logging(monkeypatch):
    neyra = Neyra()
    # Simple processing command
    monkeypatch.setattr(neyra, "process_command", lambda q: "base text")
    # gap analyzer returns a gap only once
    call_counter = {"count": 0}

    def fake_analyze(draft):
        if call_counter["count"] == 0:
            call_counter["count"] += 1
            return [KnowledgeGap(claim="gap", questions=[], confidence=0.0)]
        return []

    monkeypatch.setattr(neyra.gap_analyzer, "analyze", fake_analyze)
    monkeypatch.setattr(neyra.deep_searcher, "search", lambda *a, **k: [])
    monkeypatch.setattr(
        neyra.response_enhancer,
        "enhance",
        lambda text, results, integration, self_correct=True: text + " improved",
    )
    monkeypatch.setattr(neyra.iteration_controller, "should_iterate", lambda text: False)

    updates = []

    def fake_update(stage, iteration=None):
        updates.append((stage, iteration))

    monkeypatch.setattr("src.core.neyra_brain.update_progress", fake_update)

    neyra.iterative_response("query")

    assert updates[0] == ("start", None)
    assert ("iteration", 1) in updates
    assert updates[-1] == ("finished", 1)
