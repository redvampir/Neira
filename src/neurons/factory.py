from __future__ import annotations

from typing import Any, Dict, Type
from pathlib import Path
import importlib.util
import inspect

from .base import Neuron


class NeuronFactory:
    """Factory for registering and creating neuron classes."""

    _registry: Dict[str, Type[Neuron]] = {}
    plugins_dir: str = "neurons/plugins"

    @classmethod
    def load_plugins(cls, plugins_dir: str | None = None) -> None:
        """Load neuron plugins from ``plugins_dir``.

        Parameters
        ----------
        plugins_dir:
            Directory containing plugin modules. Uses
            :pyattr:`NeuronFactory.plugins_dir` by default.
        """

        directory = Path(plugins_dir or cls.plugins_dir)
        if not directory.is_absolute():
            directory = Path(__file__).resolve().parents[2] / directory
        if not directory.exists():
            return

        for file in directory.glob("*.py"):
            spec = importlib.util.spec_from_file_location(
                f"neuron_plugin_{file.stem}", file
            )
            if spec and spec.loader:
                module = importlib.util.module_from_spec(spec)
                spec.loader.exec_module(module)
                for _, obj in inspect.getmembers(module, inspect.isclass):
                    if issubclass(obj, Neuron) and obj is not Neuron:
                        neuron_type = getattr(obj, "type", obj.__name__)
                        cls.register(neuron_type, obj)

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

NeuronFactory.load_plugins()

__all__ = ["NeuronFactory"]
