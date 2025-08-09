"""Simple turn based scenario engine."""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import Callable, List, Tuple, Any


@dataclass(order=True)
class _Event:
    turn: int
    callback: Callable[..., Any] = field(compare=False)
    args: Tuple[Any, ...] = field(default_factory=tuple, compare=False)
    kwargs: dict = field(default_factory=dict, compare=False)


class ScenarioEngine:
    """Engine that schedules and runs events turn by turn."""

    def __init__(self) -> None:
        self.current_turn = 0
        self._queue: List[_Event] = []

    def schedule(
        self, callback: Callable[..., Any], delay: int = 0, *args: Any, **kwargs: Any
    ) -> None:
        """Schedule ``callback`` to be executed after ``delay`` turns."""

        event = _Event(self.current_turn + delay, callback, args, kwargs)
        self._queue.append(event)
        self._queue.sort()

    def run_turn(self) -> None:
        """Advance time by one turn and process due events."""

        self.current_turn += 1
        due = [e for e in self._queue if e.turn <= self.current_turn]
        self._queue = [e for e in self._queue if e.turn > self.current_turn]
        for event in due:
            event.callback(*event.args, **event.kwargs)

    def clear(self) -> None:
        """Remove all scheduled events."""

        self._queue.clear()
        self.current_turn = 0
