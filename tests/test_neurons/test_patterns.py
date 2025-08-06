"""Tests for behavior patterns combining different neuron types."""

import sys
from pathlib import Path

# Ensure project root on path for src layout
sys.path.append(str(Path(__file__).resolve().parents[2]))

from src.neurons import BehaviorPattern, MemoryNeuron, AnalysisNeuron, ActionNeuron


def test_behavior_pattern_execute_and_strength() -> None:
    memory = MemoryNeuron(id="m1")
    analysis = AnalysisNeuron(id="a1")
    action = ActionNeuron(id="act1")
    pattern = BehaviorPattern(
        memory_neurons=[memory],
        analysis_neurons=[analysis],
        action_neurons=[action],
        usage_frequency=2,
        success_rate=0.5,
    )

    result = pattern.execute("hello world")
    pattern.record_result(True)

    # Memory neuron should store the data under its key
    assert memory.query("hello world") == "hello world"

    # Action stage should be executed based on analysis results
    assert result == ["action:{'length': 11, 'words': 2}"]

    # usage_frequency and success_rate are updated
    assert pattern.usage_frequency == 3
    assert pattern.success_rate > 0.5
    assert pattern.get_strength() > 1.5


def test_successful_runs_increase_strength() -> None:
    memory = MemoryNeuron(id="m1")
    analysis = AnalysisNeuron(id="a1")
    action = ActionNeuron(id="act1")
    pattern = BehaviorPattern(
        memory_neurons=[memory],
        analysis_neurons=[analysis],
        action_neurons=[action],
    )

    strengths = []
    for _ in range(3):
        pattern.execute("ping")
        pattern.record_result(True)
        strengths.append(pattern.get_strength())

    assert strengths[0] < strengths[1] < strengths[2]

