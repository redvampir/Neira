from dataclasses import dataclass, field
from typing import Any, Callable


@dataclass
class WatchNode:
    """Node that watches the value produced by a callable."""

    name: str
    getter: Callable[[], Any]
    value: Any = field(init=False, default=None)

    def update(self) -> None:
        """Update the stored value by calling the getter."""
        self.value = self.getter()

    def display(self) -> str:
        """Return a human readable representation of the current value."""
        return f"{self.name}: {self.value}"
