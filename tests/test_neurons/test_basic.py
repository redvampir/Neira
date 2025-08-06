"""Tests for neuron classes."""

import sys
from datetime import timedelta
from pathlib import Path

# Ensure project root on path for src layout
sys.path.append(str(Path(__file__).resolve().parents[2]))

from src.neurons import Neuron, MemoryNeuron, AnalysisNeuron, ActionNeuron


def test_activation_updates_strength_and_count() -> None:
    neuron = Neuron(id="n1", type="generic")
    start_strength = neuron.strength
    neuron.activate()
    assert neuron.activation_count == 1
    assert neuron.strength > start_strength

    prev_time = neuron.last_used
    # simulate time passing to trigger decay
    neuron.last_used -= timedelta(minutes=10)
    neuron.activate()
    assert neuron.activation_count == 2
    assert neuron.last_used > prev_time
    assert neuron.strength <= 1.0


def test_memory_neuron_process() -> None:
    neuron = MemoryNeuron(id="m1")
    neuron.process("key1", "remember this")
    assert neuron.query("key1") == "remember this"
    assert neuron.activation_count == 1


def test_memory_neuron_bounded_storage() -> None:
    neuron = MemoryNeuron(id="m2", hot_limit=2, warm_limit=3)
    for i in range(6):
        neuron.process(f"k{i}", f"v{i}")
    assert len(neuron.index.hot_cache) <= 2
    assert len(neuron.index.warm_cache) <= 3
    assert len(neuron.index.cold_storage) >= 1
    neuron.purge_cold_storage()
    assert neuron.index.cold_storage == {}


def test_analysis_neuron_process() -> None:
    neuron = AnalysisNeuron(id="a1")
    result = neuron.process("hello world")
    assert result["length"] == len("hello world")
    assert result["words"] == 2
    assert neuron.activation_count == 1


def test_action_neuron_process() -> None:
    neuron = ActionNeuron(id="act1")
    outcome = neuron.process("jump")
    assert outcome == "action:jump"
    assert neuron.activation_count == 1
