"""Neuron evolution utilities.

This module analyses neuron activity and creates specialised neurons when
certain thresholds are met. Newly created neurons are slight mutations of
existing neuron types and are linked back to their source neuron.
"""

from __future__ import annotations

from dataclasses import dataclass
import random
from typing import Optional, Tuple, Type

from .base import Neuron
from .memory import MemoryNeuron
from .analysis import AnalysisNeuron
from .action import ActionNeuron
from .planning import PlanningNeuron


@dataclass
class EvolutionConfig:
    """Configuration for neuron evolution."""

    activation_threshold: int = 1
    strength_threshold: float = 0.5
    mutation_rate: float = 0.1


SPECIALISED = [MemoryNeuron, AnalysisNeuron, ActionNeuron, PlanningNeuron]


def register_base_class(cls: Type[Neuron]) -> None:
    """Register an additional base class for evolution."""

    if cls not in SPECIALISED:
        SPECIALISED.append(cls)


def evolve(
    source: Neuron,
    config: EvolutionConfig | None = None,
) -> Optional[Tuple[str, Type[Neuron]]]:
    """Evolve ``source`` into a specialised neuron if activity warrants.

    Parameters
    ----------
    source:
        The neuron whose activity is analysed.
    config:
        Optional :class:`EvolutionConfig` controlling thresholds and mutation.

    Returns
    -------
    Optional[Tuple[str, Type[Neuron]]]
        ``(neuron_type, neuron_cls)`` if evolution occurred otherwise ``None``.
    """

    cfg = config or EvolutionConfig()
    if (
        source.activation_count < cfg.activation_threshold
        and source.strength < cfg.strength_threshold
    ):
        return None

    base_cls = random.choice(SPECIALISED)
    mutated_strength = max(
        0.0,
        min(1.0, source.strength + random.uniform(-cfg.mutation_rate, cfg.mutation_rate)),
    )

    if base_cls is MemoryNeuron:
        def __init__(self, *args, strength=mutated_strength, **kwargs):
            base_cls.__init__(self, *args, strength=strength, **kwargs)
    else:
        def __init__(self, *args, strength=mutated_strength, **kwargs):  # type: ignore[misc]
            base_cls.__init__(self, *args, **kwargs)
            self.strength = strength

    neuron_type = f"{base_cls.type}_{source.id}_{source.activation_count}"
    neuron_cls = type(neuron_type, (base_cls,), {"__init__": __init__, "type": neuron_type})

    instance = neuron_cls(id=neuron_type, type=neuron_type)
    source.connect(instance)
    return neuron_type, neuron_cls


__all__ = ["EvolutionConfig", "evolve", "register_base_class"]
