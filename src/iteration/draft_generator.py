"""Utilities for building quick drafts based on existing memories."""

from __future__ import annotations

from typing import Any

from src.memory import CharacterMemory, WorldMemory, StyleMemory, MemoryIndex


class DraftGenerator:
    """Generate draft responses using cached memory objects.

    The generator attempts a fast lookup in ``hot_memory`` before falling back
    to the slower persistent memories like :class:`CharacterMemory`,
    :class:`WorldMemory` and :class:`StyleMemory`.
    """

    def __init__(
        self,
        character_memory: CharacterMemory | None = None,
        world_memory: WorldMemory | None = None,
        style_memory: StyleMemory | None = None,
    ) -> None:
        self.character_memory = character_memory or CharacterMemory()
        self.world_memory = world_memory or WorldMemory()
        self.style_memory = style_memory or StyleMemory()

    # ------------------------------------------------------------------
    def generate_draft(self, query: str, hot_memory: MemoryIndex | Any) -> str:
        """Return a quick draft for ``query`` using ``hot_memory``.

        Parameters
        ----------
        query:
            Key describing the information of interest.
        hot_memory:
            The fast tier cache, typically a :class:`MemoryIndex` instance.
        """

        if hasattr(hot_memory, "get"):
            try:
                result = hot_memory.get(query)
            except Exception:  # pragma: no cover - defensive
                result = None
            if result:
                return str(result)

        try:
            char = self.character_memory.get(query)
        except Exception:  # pragma: no cover - defensive
            char = None
        if char:
            appearance = getattr(char, "appearance", "")
            return f"{char.name}: {appearance}" if appearance else char.name

        try:
            world_info = self.world_memory.get(query)
        except Exception:  # pragma: no cover - defensive
            world_info = None
        if world_info:
            return str(world_info)

        try:
            examples = self.style_memory.get_examples("default")
        except Exception:  # pragma: no cover - defensive
            examples = []
        if examples:
            return examples[0]

        return ""


__all__ = ["DraftGenerator"]
