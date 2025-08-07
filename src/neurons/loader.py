from __future__ import annotations

import json
from importlib import import_module
from pathlib import Path
from typing import Any

from .factory import NeuronFactory


def load_neurons(path: Path | str = Path("data/neurons")) -> None:
    """Load persisted neuron classes from ``path`` and register them."""
    neuron_dir = Path(path)
    if not neuron_dir.exists():
        return

    for file in neuron_dir.glob("*.json"):
        data = json.loads(file.read_text())
        neuron_type = data.get("neuron_type") or file.stem
        base_path = data.get("base_class")
        if not base_path:
            continue
        module_name, class_name = base_path.rsplit(".", 1)
        module = import_module(module_name)
        base_cls = getattr(module, class_name)
        strength = data.get("strength", 0.5)

        def _make_init(base_cls: type, strength: float):
            def __init__(self, *args: Any, strength: float = strength, **kwargs: Any):
                base_cls.__init__(self, *args, **kwargs)
                self.strength = strength
            return __init__

        init = _make_init(base_cls, strength)
        neuron_cls = type(neuron_type, (base_cls,), {"__init__": init, "type": neuron_type})
        NeuronFactory.register(neuron_type, neuron_cls)
