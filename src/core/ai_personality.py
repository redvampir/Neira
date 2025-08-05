"""AI personality definition.

This module defines the :class:`AIPersonality` dataclass which models
configurable characteristics for an AI agent.  It includes convenience
methods for updating memory and communicating with other personalities.
"""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import Dict, List


@dataclass
class AIPersonality:
    """Represents an AI personality with context and behaviour traits."""

    name: str
    role: str
    knowledge_focus: List[str]
    personality_traits: List[str]
    current_character: str
    memory_context: Dict[str, List[str]] = field(default_factory=dict)
    decision_style: str = ""
    communication_style: str = ""

    def update_memory(self, context: str, information: str) -> None:
        """Store ``information`` under the specified ``context`` key."""
        self.memory_context.setdefault(context, []).append(information)

    def communicate(self, message: str, other: "AIPersonality") -> str:
        """Return a simple message directed at ``other`` personality."""
        return f"{self.name} to {other.name}: {message}"

