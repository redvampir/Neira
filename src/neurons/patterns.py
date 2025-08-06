"""Behavior pattern orchestrating groups of neurons."""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import Any, List

from .memory import MemoryNeuron
from .analysis import AnalysisNeuron
from .action import ActionNeuron


@dataclass
class BehaviorPattern:
    """Composite pattern that executes a sequence of neurons.

    The pattern is composed of three stages:

    1. Activation of memory neurons.
    2. Running analysis neurons.
    3. Triggering action neurons based on analysis results.

    Each stage can contain an arbitrary number of neurons. The class also
    tracks how often it was used and how successful it was in order to
    estimate the pattern's strength.
    """

    memory_neurons: List[MemoryNeuron] = field(default_factory=list)
    analysis_neurons: List[AnalysisNeuron] = field(default_factory=list)
    action_neurons: List[ActionNeuron] = field(default_factory=list)
    usage_frequency: int = 0
    success_rate: float = 0.0

    # ------------------------------------------------------------------
    def execute(self, data: Any) -> List[Any]:
        """Run the pattern through memory, analysis and action stages.

        Parameters
        ----------
        data:
            Input passed to memory and analysis neurons. Action neurons
            receive stringified analysis results.

        Returns
        -------
        List[Any]
            Outputs produced by action neurons.
        """

        # Memory activation stage
        for neuron in self.memory_neurons:
            neuron.process(str(data), data)

        # Analysis stage
        analysis_results: List[Any] = []
        for neuron in self.analysis_neurons:
            analysis_results.append(neuron.process(data))

        # Action stage
        actions: List[Any] = []
        for neuron in self.action_neurons:
            for result in analysis_results:
                actions.append(neuron.process(str(result)))

        self.usage_frequency += 1
        return actions

    # ------------------------------------------------------------------
    def record_result(self, success: bool, alpha: float = 0.2) -> None:
        """Update ``success_rate`` based on the latest outcome.

        The rate is tracked using an exponential moving average so that more
        recent executions have a stronger impact while still keeping the
        historical performance in consideration.

        Parameters
        ----------
        success:
            Whether the latest execution was successful.
        alpha:
            Smoothing factor for the exponential moving average. A higher
            value makes the metric react faster to new results.
        """

        outcome = 1.0 if success else 0.0
        self.success_rate = self.success_rate * (1 - alpha) + outcome * alpha

    # ------------------------------------------------------------------
    def get_strength(self) -> float:
        """Return an aggregate measure of the pattern's significance."""

        return self.usage_frequency * self.success_rate

