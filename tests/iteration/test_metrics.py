import logging

from src.core.neyra_brain import Neyra
from src.iteration import KnowledgeGap
from src.iteration.metrics import similarity, length, corrected_errors


def test_metric_functions():
    assert similarity("same", "same") == 1.0
    assert similarity("abc", "abd") < 1.0
    assert length("one two three") == 3
    assert corrected_errors("I has apple", "I have an apple") == 2


def test_logging_during_iteration(monkeypatch, caplog):
    neyra = Neyra()
    monkeypatch.setattr(neyra, "process_command", lambda q: "base text")
    monkeypatch.setattr(
        neyra.gap_analyzer,
        "analyze",
        lambda draft: [KnowledgeGap(claim="gap", questions=[], confidence=0.0)],
    )
    monkeypatch.setattr(neyra.deep_searcher, "search", lambda *a, **k: [])
    monkeypatch.setattr(
        neyra.response_enhancer,
        "enhance",
        lambda text, results, integration, self_correct=True: text + " improved",
    )
    calls = {"count": 0}

    def fake_iterate(text):
        calls["count"] += 1
        return calls["count"] < 2

    monkeypatch.setattr(neyra.iteration_controller, "should_iterate", fake_iterate)

    with caplog.at_level(logging.INFO):
        neyra.iterative_response("query")

    assert "Iteration 1 metrics" in caplog.text
    assert "Iteration 2 metrics" in caplog.text
