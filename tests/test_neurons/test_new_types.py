from __future__ import annotations

from src.learning import LearningSystem
from src.neurons import Neuron, MemoryNeuron, PlanningNeuron
from src.neurons.evolution import EvolutionConfig, evolve


def test_create_new_neuron_type_tracks_categories(monkeypatch) -> None:
    system = LearningSystem()
    system.failure_analysis = [
        {"error_type": "logical"},
        {"error_type": "linguistic"},
        {"error_type": "logical"},
        {"error_type": "system"},
    ]

    monkeypatch.setattr(
        "src.learning.learning_system.evolve",
        lambda source, cfg: ("dummy", MemoryNeuron),
    )

    neuron_type = system.create_new_neuron_type()
    assert neuron_type == "dummy"
    assert system.error_categories == {"logical": 2, "linguistic": 1, "system": 1}


def test_evolve_supports_planning_neuron(monkeypatch) -> None:
    source = Neuron(id="src", type="base", activation_count=3, strength=0.9)
    cfg = EvolutionConfig(activation_threshold=2, strength_threshold=0.8)

    monkeypatch.setattr(
        "src.neurons.evolution.random.choice", lambda seq: PlanningNeuron
    )

    result = evolve(source, cfg)
    assert result is not None
    _, neuron_cls = result
    new_neuron = source.connections[0]
    assert isinstance(new_neuron, PlanningNeuron)
