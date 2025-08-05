"""Utilities for creating and tracking AI personalities.

This module provides helper factory functions to build common ``AIPersonality``
configurations used throughout the project.  Each created personality is stored
in a module level registry keyed by a unique integer identifier.  The registry
allows tests and other modules to access active personalities for coordination
or inspection.
"""

from __future__ import annotations

from itertools import count
from typing import Dict

from .ai_personality import AIPersonality

# ---------------------------------------------------------------------------
# Registry management
# ---------------------------------------------------------------------------
# A simple in-memory registry for all personalities created via this module.
# The key is a unique integer identifier assigned sequentially.
active_personalities: Dict[int, AIPersonality] = {}
_id_counter = count(1)


def _register(personality: AIPersonality) -> AIPersonality:
    """Store ``personality`` in the registry and assign a unique id.

    The assigned id is also attached to the ``AIPersonality`` instance as the
    attribute ``id`` for easy reference.  The function returns the personality
    instance to allow fluent usage in factory functions.
    """

    pid = next(_id_counter)
    active_personalities[pid] = personality
    # ``AIPersonality`` does not define an ``id`` field but Python dataclasses
    # are mutable, so we attach it dynamically for convenience.
    setattr(personality, "id", pid)
    return personality


# ---------------------------------------------------------------------------
# Factory functions
# ---------------------------------------------------------------------------

def create_master_personality(name: str = "Master", **overrides) -> AIPersonality:
    """Create the master/game controller personality.

    Parameters
    ----------
    name:
        Optional name for the master personality.  Defaults to ``"Master"``.
    overrides:
        Additional keyword arguments to override default personality fields.

    Returns
    -------
    AIPersonality
        The created master personality registered in ``active_personalities``.
    """

    defaults = {
        "name": name,
        "role": "master",
        "knowledge_focus": ["global overview"],
        "personality_traits": ["authoritative", "balanced"],
        "current_character": "master",
        "decision_style": "strategic",
        "communication_style": "directive",
    }
    defaults.update(overrides)
    personality = AIPersonality(**defaults)
    return _register(personality)


def create_player_personality(name: str, **overrides) -> AIPersonality:
    """Create a personality representing a player.

    Parameters
    ----------
    name:
        Name of the player personality.  Used as the default character name.
    overrides:
        Additional keyword arguments to override default personality fields.

    Returns
    -------
    AIPersonality
        The created player personality registered in ``active_personalities``.
    """

    defaults = {
        "name": name,
        "role": "player",
        "knowledge_focus": [],
        "personality_traits": [],
        "current_character": name,
        "decision_style": "personal",
        "communication_style": "casual",
    }
    defaults.update(overrides)
    personality = AIPersonality(**defaults)
    return _register(personality)


def create_specialist_personality(
    name: str, speciality: str, **overrides
) -> AIPersonality:
    """Create a specialist personality focused on a particular topic.

    Parameters
    ----------
    name:
        Name of the specialist.
    speciality:
        Area of expertise for the personality.  This becomes both the role and
        the sole default entry in ``knowledge_focus``.
    overrides:
        Additional keyword arguments to override default personality fields.

    Returns
    -------
    AIPersonality
        The created specialist personality registered in
        ``active_personalities``.
    """

    defaults = {
        "name": name,
        "role": "specialist",
        "knowledge_focus": [speciality],
        "personality_traits": [],
        "current_character": speciality,
        "decision_style": "analytical",
        "communication_style": "formal",
    }
    defaults.update(overrides)
    personality = AIPersonality(**defaults)
    return _register(personality)


# ---------------------------------------------------------------------------
# Helper accessors
# ---------------------------------------------------------------------------

def get_personality(pid: int) -> AIPersonality | None:
    """Retrieve a personality by its unique ``pid``."""
    return active_personalities.get(pid)


def remove_personality(pid: int) -> bool:
    """Remove a personality from the registry.

    Returns ``True`` if a personality was removed, ``False`` otherwise.
    """
    return active_personalities.pop(pid, None) is not None


def list_personalities() -> Dict[int, AIPersonality]:
    """Return a shallow copy of the active personalities registry."""
    return dict(active_personalities)
