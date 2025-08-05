"""Memory related neuron."""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import Any, List

from .base import Neuron


@dataclass
class MemoryNeuron(Neuron):
    """Neuron specialised in storing information."""

    type: str = "memory"
    memory: List[Any] = field(default_factory=list)

    # ------------------------------------------------------------------
    def process(self, data: Any) -> None:
        """Store incoming data to internal memory."""

        self.activate()
        self.memory.append(data)
