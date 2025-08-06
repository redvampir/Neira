from __future__ import annotations

from typing import Any, Dict, Type

from .base import Neuron


class NeuronFactory:
    """Factory for registering and creating neuron classes."""

    _registry: Dict[str, Type[Neuron]] = {}

    @classmethod
    def register(cls, neuron_type: str, neuron_cls: Type[Neuron]) -> None:
        """Register ``neuron_cls`` under ``neuron_type``."""

        cls._registry[neuron_type] = neuron_cls

    @classmethod
    def create(cls, neuron_type: str, *args: Any, **kwargs: Any) -> Neuron:
        """Instantiate the neuron associated with ``neuron_type``."""

        neuron_cls = cls._registry.get(neuron_type)
        if neuron_cls is None:
            raise ValueError(f"Unknown neuron_type: {neuron_type}")
        return neuron_cls(*args, **kwargs)


__all__ = ["NeuronFactory"]
