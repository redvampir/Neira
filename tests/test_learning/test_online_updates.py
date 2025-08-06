from __future__ import annotations

from src.learning import LearningSystem, OnlineLearningEngine


def test_online_learning_engine_integration(monkeypatch) -> None:
    system = LearningSystem()
    sources = ["source1", "source2"]
    data = {
        "source1": [
            {"title": "Useful update", "content": "info", "quality": 0.9},
            {"title": "Buy now", "content": "spam", "quality": 0.9},
        ],
        "source2": [
            {"title": "Low quality", "content": "text", "quality": 0.1}
        ],
    }

    def fake_fetch(url: str):
        return data[url]

    engine = OnlineLearningEngine(
        sources,
        spam_keywords=["buy"],
        quality_threshold=0.5,
        fetch_func=fake_fetch,
    )

    added = engine.integrate(system)
    assert added == 1
    entry = system.knowledge_base._entries[0]
    assert entry["title"] == "Useful update"
    assert entry["update_label"] == "online_update"
