from __future__ import annotations

import sys
from pathlib import Path

sys.path.append(str(Path(__file__).resolve().parents[2]))

from src.core.migration import Migrator, MigrationStep
from src.core.state_manager import StateManager


def test_migration_applied_on_version_change():
    def step(state):
        state["value"] += 1
        return state

    migrator = Migrator()
    migrator.add_step(MigrationStep((1, 0), (1, 1), step))

    manager = StateManager(version=(1, 0), migrator=migrator)
    manager.register("value", 1)

    manager.set_version((1, 1))
    assert manager.version == (1, 1)
    assert manager.get("value") == 2
