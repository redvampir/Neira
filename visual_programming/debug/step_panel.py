from dataclasses import dataclass, field
from typing import Any, Callable, Dict, List

from ..watch import WatchNode


@dataclass
class StepDebugger:
    """Simple step-by-step debugger for visual programming."""

    steps: List[Callable[[], None]] = field(default_factory=list)
    watches: Dict[str, WatchNode] = field(default_factory=dict)
    current_step: int = 0

    def add_step(self, func: Callable[[], None]) -> None:
        """Register a callable to be executed on each step."""
        self.steps.append(func)

    def add_watch(self, name: str, getter: Callable[[], Any]) -> WatchNode:
        """Attach a watch node that displays the value from ``getter``."""
        node = WatchNode(name, getter)
        node.update()
        self.watches[name] = node
        return node

    def step(self) -> None:
        """Execute the current step and refresh watches."""
        if self.current_step >= len(self.steps):
            raise StopIteration("No more steps available")
        self.steps[self.current_step]()
        for node in self.watches.values():
            node.update()
        self.current_step += 1

    def reset(self) -> None:
        """Reset the step counter and update watches."""
        self.current_step = 0
        for node in self.watches.values():
            node.update()
