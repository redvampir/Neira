from __future__ import annotations

from src.neurons import Neuron, MemoryNeuron, AnalysisNeuron, ActionNeuron
from src.neurons.evolution import EvolutionConfig, evolve


def test_evolve_creates_specialised_neuron_and_links() -> None:
    source = Neuron(id="src", type="base", activation_count=3, strength=0.9)
    cfg = EvolutionConfig(activation_threshold=2, strength_threshold=0.8)
    result = evolve(source, cfg)
    assert result is not None
    neuron_type, neuron_cls = result
    assert source.connections
    new_neuron = source.connections[0]
    assert new_neuron.type == neuron_type
    assert isinstance(new_neuron, (MemoryNeuron, AnalysisNeuron, ActionNeuron))
    assert new_neuron.strength != 0.5
