from .cache_manager import CacheManager
from .ai_personality import AIPersonality
from .personality_context import PersonalityContext
from .session_memory import SessionMemory
try:  # pragma: no cover - optional dependency
    from .adaptive_loader import (
        enable,
        disable,
        determine_active_components,
        resource_manager,
    )
except Exception:  # pragma: no cover - fallback when optional modules missing
    def enable(*_args, **_kwargs):
        return None

    def disable(*_args, **_kwargs):
        return None

    def determine_active_components(*_args, **_kwargs):
        return []

    resource_manager = None
from .event_bus import Event, EventBus
from .priority_scheduler import Priority, PriorityScheduler

__all__ = [
    "CacheManager",
    "AIPersonality",
    "PersonalityContext",
    "SessionMemory",
    "enable",
    "disable",
    "determine_active_components",
    "resource_manager",
    "Event",
    "EventBus",
    "Priority",
    "PriorityScheduler",
]
