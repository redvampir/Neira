from __future__ import annotations

from src.learning import KnowledgeBase, LearningSystem


def test_knowledge_base_add_query_save(tmp_path) -> None:
    path = tmp_path / "kb.json"
    kb = KnowledgeBase(path)
    entry = {"request": "hi", "description": "logical"}
    kb.add(entry)
    kb.save()

    loaded = KnowledgeBase(path)
    assert loaded.query("hi") == entry


def test_learning_system_persists_failures(tmp_path) -> None:
    path = tmp_path / "kb.json"
    system = LearningSystem()
    system.knowledge_base = KnowledgeBase(path)
    ctx = {"start_time": 0.0, "end_time": 1.0}
    system.learn_from_interaction("hi", "oops", -1, ctx)

    other = LearningSystem()
    other.knowledge_base = KnowledgeBase(path)
    failure = other.check_previous_failures("hi")
    assert failure is not None
    assert failure["request"] == "hi"
