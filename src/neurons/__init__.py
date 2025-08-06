"""Neuron classes for core processing.

This module exposes base and specialized neuron types used within
Neira's internal reasoning systems.
"""

from .base import Neuron
from .memory import MemoryNeuron
from .analysis import AnalysisNeuron
from .action import ActionNeuron
from .planning import PlanningNeuron
from .patterns import BehaviorPattern
from .network import NeuronNetwork
from .factory import NeuronFactory
from .evolution import EvolutionConfig, evolve

__all__ = [
    "Neuron",
    "MemoryNeuron",
    "AnalysisNeuron",
    "ActionNeuron",
    "PlanningNeuron",
    "BehaviorPattern",
    "NeuronNetwork",
    "NeuronFactory",
    "EvolutionConfig",
    "evolve",
]
