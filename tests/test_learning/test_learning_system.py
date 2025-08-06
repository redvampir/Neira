from __future__ import annotations

from src.learning import LearningSystem
from src.neurons import NeuronFactory
from src.memory import StyleMemory


def test_learn_from_interaction_records_metrics() -> None:
    system = LearningSystem()
    ctx = {"start_time": 0.0, "end_time": 1.5}
    system.learn_from_interaction("hi", "hello", -1, ctx)

    interaction = system.experience_buffer[0]
    metrics = interaction["metrics"]
    assert metrics["success"] is False
    assert metrics["reaction_time"] == 1.5
    assert metrics["error_type"] is not None


def test_request_cache_and_success_rate() -> None:
    system = LearningSystem()
    ctx = {"start_time": 0.0, "end_time": 0.1}
    system.learn_from_interaction("good", "yes", 1, ctx)
    system.learn_from_interaction("bad", "no", -1, ctx)

    assert system.adaptation_weights["success_rate"] == 0.5

    system.response_cache["cached"] = "stored"
    assert system.get_cached_response("cached") == "stored"


def test_create_new_neuron_type_registers() -> None:
    system = LearningSystem()
    ctx = {"start_time": 0.0, "end_time": 0.1}
    system.learn_from_interaction("hi", "hello", -1, ctx)
    neuron_type = system.create_new_neuron_type()
    neuron = NeuronFactory.create(neuron_type, id="n1", type=neuron_type)
    assert neuron.type == neuron_type


def test_save_and_load_state_roundtrip(tmp_path) -> None:
    system = LearningSystem()
    ctx = {"start_time": 0.0, "end_time": 0.1}
    system.learn_from_interaction("hi", "hello", 1, ctx)
    path = tmp_path / "state.json"
    system.save_state(path)
    loaded = LearningSystem.load_state(path)
    assert loaded.experience_buffer == system.experience_buffer
    assert loaded.success_metrics == system.success_metrics
    assert loaded.failure_analysis == system.failure_analysis


def test_positive_feedback_saves_user_style(tmp_path) -> None:
    system = LearningSystem()
    system.style_memory = StyleMemory(tmp_path / "styles.json")
    context = {
        "user_id": "u1",
        "tone": "дружелюбный",
        "examples": ["пример"],
        "start_time": 0.0,
        "end_time": 1.0,
    }
    system.learn_from_interaction("hi", "hello", 1, context)
    pattern = system.style_memory.get_style("u1", "preferred")
    assert pattern is not None
    assert pattern.description == "дружелюбный"
    assert "пример" in pattern.examples

