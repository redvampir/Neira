"""Base neuron implementation."""

from __future__ import annotations

from dataclasses import dataclass, field
from datetime import datetime
from typing import List, Any


@dataclass
class Neuron:
    """Basic building block for reasoning structures.

    Parameters
    ----------
    id:
        Unique identifier for the neuron.
    type:
        Logical type of the neuron (e.g. memory, analysis).
    connections:
        Other neurons this neuron is linked to.
    activation_count:
        Number of times the neuron has been activated.
    strength:
        Relative importance of the neuron. The value is automatically
        regulated using :pyattr:`activation_count` and last usage time.
    last_used:
        Timestamp of the last activation.
    """

    id: str
    type: str
    connections: List["Neuron"] = field(default_factory=list)
    activation_count: int = 0
    strength: float = 0.5
    last_used: datetime = field(default_factory=datetime.utcnow)

    # ------------------------------------------------------------------
    def connect(self, neuron: "Neuron") -> None:
        """Connect this neuron to another neuron."""

        if neuron not in self.connections:
            self.connections.append(neuron)

    # ------------------------------------------------------------------
    def activate(self) -> None:
        """Activate the neuron and update its internal statistics."""

        self.activation_count += 1
        self.update_strength()

    # ------------------------------------------------------------------
    def update_strength(self) -> None:
        """Adjust the neuron's strength based on usage.

        The strength increases with every activation with diminishing
        returns while decaying slowly over time if the neuron is not
        used. This keeps frequently used neurons more influential while
        letting rarely used ones fade.
        """

        now = datetime.utcnow()
        elapsed = (now - self.last_used).total_seconds()
        # Decay strength by 1% per minute since last use
        self.strength *= 0.99 ** (elapsed / 60)
        # Reinforce based on activation count with diminishing returns
        self.strength += 1 / (self.activation_count + 1)
        self.strength = max(0.0, min(self.strength, 1.0))
        self.last_used = now

    # ------------------------------------------------------------------
    def process(self, *args: Any, **kwargs: Any) -> Any:  # pragma: no cover - abstract
        """Placeholder for subclasses to implement their behaviour."""

        raise NotImplementedError
