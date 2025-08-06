from __future__ import annotations

from src.learning import LearningSystem
from src.neurons import NeuronFactory


def test_learn_from_interaction_updates_metrics() -> None:
    system = LearningSystem()
    system.learn_from_interaction("hi", "hello", -1, {"topic": "combat"})
    assert len(system.experience_buffer) == 1
    assert system.success_metrics["negative"] == 1
    assert system.failure_analysis["combat"] == 1


def test_create_new_neuron_type_registers() -> None:
    system = LearningSystem()
    system.learn_from_interaction("hi", "hello", -1, {"topic": "magic"})
    neuron_type = system.create_new_neuron_type()
    neuron = NeuronFactory.create(neuron_type, id="n1", type=neuron_type)
    assert neuron.type == neuron_type


def test_save_and_load_state_roundtrip(tmp_path) -> None:
    system = LearningSystem()
    system.learn_from_interaction("hi", "hello", 1, {"topic": "lore"})
    path = tmp_path / "state.json"
    system.save_state(path)
    loaded = LearningSystem.load_state(path)
    assert loaded.experience_buffer == system.experience_buffer
    assert loaded.success_metrics == system.success_metrics
    assert loaded.failure_analysis == system.failure_analysis
