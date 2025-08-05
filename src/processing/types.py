from __future__ import annotations

from dataclasses import dataclass, field
from enum import Enum, auto
from typing import Any, Callable, Tuple, Dict


class Priority(Enum):
    """Priority levels for tasks."""

    HIGH = auto()
    MEDIUM = auto()
    LOW = auto()


@dataclass
class Task:
    """A unit of work to be processed."""

    func: Callable[..., Any]
    args: Tuple[Any, ...] = field(default_factory=tuple)
    kwargs: Dict[str, Any] = field(default_factory=dict)

    def run(self) -> Any:
        """Execute the task and return the result."""

        return self.func(*self.args, **self.kwargs)
