"""Manage multiple AI personalities and their knowledge separation."""

from __future__ import annotations

from typing import Any, Dict, Iterable, Set


class PersonalityContext:
    """Store and switch between multiple AI personalities.

    Each personality has an isolated knowledge base but knowledge can be
    selectively shared with others.
    """

    def __init__(self) -> None:
        self._personalities: Dict[str, Dict[str, Any]] = {}
        self.current: str | None = None

    def switch_to_personality(self, personality_id: str) -> str:
        """Activate ``personality_id`` and create it if missing."""
        self.current = personality_id
        self._personalities.setdefault(personality_id, {"knowledge": set()})
        return personality_id

    def maintain_separation(self) -> Set[Any]:
        """Return a snapshot of the current personality's knowledge base."""
        if self.current is None:
            return set()
        knowledge: Set[Any] = self._personalities[self.current]["knowledge"]
        return set(knowledge)

    def share_knowledge(
        self,
        knowledge: Iterable[Any],
        target_personalities: Iterable[str] | None = None,
    ) -> None:
        """Share ``knowledge`` with other personalities.

        ``target_personalities`` specifies which personalities should receive
        the knowledge.  When ``None`` all personalities except the current one
        are updated.
        """

        if self.current is None:
            return
        targets = (
            [pid for pid in self._personalities if pid != self.current]
            if target_personalities is None
            else list(target_personalities)
        )
        for pid in targets:
            self._personalities.setdefault(pid, {"knowledge": set()})
            self._personalities[pid]["knowledge"].update(knowledge)
