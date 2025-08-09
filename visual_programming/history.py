from __future__ import annotations

"""Simple change history with undo support for visual programming graphs."""

from dataclasses import dataclass, field
from typing import Any, List
import copy


@dataclass
class History:
    """Utility to record immutable snapshots and restore previous states.

    The history stores deep copies of the supplied states in a stack.  Calling
    :meth:`undo` returns the previously recorded state or ``None`` if no older
    snapshot is available.
    """

    limit: int | None = None
    _states: List[Any] = field(default_factory=list)

    def record(self, state: Any) -> None:
        """Store ``state`` as the latest snapshot."""

        self._states.append(copy.deepcopy(state))
        if self.limit is not None and len(self._states) > self.limit:
            # Drop oldest entry when exceeding the limit
            del self._states[0]

    def undo(self) -> Any | None:
        """Return the previous snapshot without altering it.

        The most recently recorded state is discarded and the new last item is
        returned.  ``None`` is returned when there is no earlier snapshot.
        """

        if len(self._states) <= 1:
            return None
        # Remove current state and return copy of the previous
        self._states.pop()
        return copy.deepcopy(self._states[-1])

    def clear(self) -> None:
        """Remove all stored states."""

        self._states.clear()


__all__ = ["History"]
