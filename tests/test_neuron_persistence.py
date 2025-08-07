from __future__ import annotations

from pathlib import Path

from src.learning import LearningSystem
from src.neurons import NeuronFactory
from src.neurons.loader import load_neurons


def test_neuron_persistence(tmp_path, monkeypatch) -> None:
    monkeypatch.chdir(tmp_path)
    system = LearningSystem()
    ctx = {"start_time": 0.0, "end_time": 0.1}
    system.learn_from_interaction("hi", "hello", -1, ctx)
    neuron_type = system.create_new_neuron_type()
    file_path = Path("data/neurons") / f"{neuron_type}.json"
    assert file_path.exists()

    NeuronFactory._registry.clear()  # type: ignore[attr-defined]
    load_neurons()
    neuron = NeuronFactory.create(neuron_type, id="n1", type=neuron_type)
    assert neuron.type == neuron_type
