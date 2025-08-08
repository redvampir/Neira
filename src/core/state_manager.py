from __future__ import annotations

"""Generic state snapshot manager.

This module provides :class:`StateManager` which offers a minimal
transaction-like interface for Python objects. Components can register
arbitrary pieces of state with the manager and then use ``begin``,
``commit`` and ``rollback`` to manage snapshots of that state.  The
snapshots are created using :func:`copy.deepcopy` so that mutations after
``begin`` do not affect the stored copy.
"""

from dataclasses import dataclass
from typing import Any, Dict, List, Optional
import copy

from .migration import Migrator, Version


@dataclass
class _Snapshot:
    """Container for a single state snapshot."""

    data: Dict[str, Any]


class StateManager:
    """Keep track of subsystem state and allow rollbacks.

    The manager stores named pieces of state in ``_state``.  Calling
    :meth:`begin` takes a deep copy of the current state and pushes it on a
    stack.  :meth:`commit` discards the most recent snapshot while
    :meth:`rollback` restores the last snapshot.

    A schema ``version`` is associated with the state.  When the version is
    changed via :meth:`set_version`, registered migration steps are executed to
    transform the state to the new format.
    """

    def __init__(
        self,
        version: Version = (1, 0),
        migrator: Optional[Migrator] = None,
    ) -> None:
        self._state: Dict[str, Any] = {}
        self._history: List[_Snapshot] = []
        self._version: Version = version
        self._migrator = migrator or Migrator()

    # ------------------------------------------------------------------
    def register(self, name: str, value: Any) -> None:
        """Register ``value`` under ``name`` in the current state."""

        self._state[name] = value

    def get(self, name: str) -> Any:
        """Return previously registered state ``name`` if present."""

        return self._state.get(name)

    # ------------------------------------------------------------------
    # transaction handling
    def begin(self) -> None:
        """Store a snapshot of the current state."""

        snapshot = copy.deepcopy(self._state)
        self._history.append(_Snapshot(snapshot))

    def commit(self) -> None:
        """Discard the last stored snapshot."""

        if not self._history:
            raise RuntimeError("no transaction to commit")
        self._history.pop()

    def rollback(self) -> None:
        """Restore the most recent snapshot."""

        if not self._history:
            raise RuntimeError("no transaction to rollback")
        snapshot = self._history.pop()
        self._state = snapshot.data

    # ------------------------------------------------------------------
    @property
    def version(self) -> Version:
        """Return the current state version."""

        return self._version

    def set_version(self, new_version: Version) -> None:
        """Update to ``new_version`` applying migrations if necessary."""

        if new_version == self._version:
            return
        self._state = self._migrator.migrate(self._state, self._version, new_version)
        self._version = new_version

    # ------------------------------------------------------------------
    @property
    def state(self) -> Dict[str, Any]:
        """Expose the current state mapping."""

        return self._state


__all__ = ["StateManager"]
