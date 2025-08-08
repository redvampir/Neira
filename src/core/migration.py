from __future__ import annotations

"""State migration utilities.

The state managed by :class:`~src.core.state_manager.StateManager` can evolve
between versions.  Versions are expressed as ``(major, minor)`` tuples of
integers, for example ``(1, 0)`` or ``(2, 5)``.  To upgrade state from one
version to another, a sequence of migration steps is executed where each step
knows how to transform the state from one specific version to the next.
"""

from dataclasses import dataclass
from typing import Any, Callable, Dict, Tuple

Version = Tuple[int, int]
MigrationFunc = Callable[[Dict[str, Any]], Dict[str, Any]]


@dataclass
class MigrationStep:
    """A single transformation from one version to another.

    ``migrate`` receives the current state mapping and returns an updated
    mapping for ``to_version``.
    """

    from_version: Version
    to_version: Version
    migrate: MigrationFunc


class Migrator:
    """Apply registered migration steps to a state mapping."""

    def __init__(self) -> None:
        self._steps: Dict[Version, MigrationStep] = {}

    def add_step(self, step: MigrationStep) -> None:
        """Register a :class:`MigrationStep`."""

        self._steps[step.from_version] = step

    def migrate(
        self, state: Dict[str, Any], from_version: Version, to_version: Version
    ) -> Dict[str, Any]:
        """Migrate ``state`` from ``from_version`` to ``to_version``.

        Steps are executed sequentially until ``to_version`` is reached.
        """

        current = from_version
        new_state = state
        while current != to_version:
            step = self._steps.get(current)
            if step is None:
                raise ValueError(
                    f"no migration step registered for {current} -> {to_version}"
                )
            new_state = step.migrate(new_state)
            current = step.to_version
        return new_state


__all__ = ["Version", "MigrationStep", "Migrator"]
