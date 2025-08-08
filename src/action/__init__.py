from .dialogue_master import DialogueMaster
from .scene_painter import ScenePainter
from .description_writer import DescriptionWriter

from src.core.state_manager import StateManager

__all__ = [
    "DialogueMaster",
    "ScenePainter",
    "DescriptionWriter",
    "action_state",
    "begin",
    "commit",
    "rollback",
]

# Global state manager for the action subsystem.  Action classes can store
# and restore their state via this manager when complex sequences require
# rollback capability.
action_state = StateManager()


def begin() -> None:
    """Create a snapshot of the action state."""

    action_state.begin()


def commit() -> None:
    """Commit changes since the last :func:`begin`."""

    action_state.commit()


def rollback() -> None:
    """Restore the action state from the last snapshot."""

    action_state.rollback()
