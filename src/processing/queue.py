from __future__ import annotations

from collections import deque
from typing import Deque, Optional, Any

from .types import Priority, Task


class ProcessingQueue:
    """A queue managing tasks with different priorities."""

    def __init__(self) -> None:
        self.high: Deque[Task] = deque()
        self.medium: Deque[Task] = deque()
        self.low: Deque[Task] = deque()

    def add_task(self, task: Task, priority: Priority = Priority.MEDIUM) -> None:
        """Add a task to the queue based on its priority."""

        if priority is Priority.HIGH:
            self.high.append(task)
        elif priority is Priority.MEDIUM:
            self.medium.append(task)
        else:
            self.low.append(task)

    def process_next(self) -> Optional[Any]:
        """Process the next task based on priority.

        Returns the result of the task execution, or ``None`` if no tasks remain.
        """

        if self.high:
            task = self.high.popleft()
        elif self.medium:
            task = self.medium.popleft()
        elif self.low:
            task = self.low.popleft()
        else:
            return None
        return task.run()
