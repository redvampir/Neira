from __future__ import annotations

"""Learn from user feedback and update persistent memories."""

from typing import Any, Dict, Iterable

from src.memory import CharacterMemory, WorldMemory, StyleMemory
from src.models import Character


class FeedbackLearner:
    """Analyse confirmed corrections and store them in memory.

    Parameters
    ----------
    characters:
        Shared :class:`~src.memory.character_memory.CharacterMemory` instance.
    worlds:
        Shared :class:`~src.memory.world_memory.WorldMemory` instance.
    styles:
        Shared :class:`~src.memory.style_memory.StyleMemory` instance.
    """

    def __init__(
        self,
        characters: CharacterMemory,
        worlds: WorldMemory,
        styles: StyleMemory,
    ) -> None:
        self.characters = characters
        self.worlds = worlds
        self.styles = styles

    # ------------------------------------------------------------------
    def apply(self, feedback: Iterable[Dict[str, Any]]) -> None:
        """Process an iterable of feedback items.

        Each item should contain ``type``, ``data`` and ``confirmed`` keys. When
        ``confirmed`` is ``True`` the corresponding memory is updated.
        """

        for item in feedback:
            if not item.get("confirmed"):
                continue
            data = item.get("data", {})
            kind = item.get("type")
            if kind == "character":
                self._update_character(data)
            elif kind == "world":
                self._update_world(data)
            elif kind == "style":
                self._update_style(data)

    # ------------------------------------------------------------------
    def _update_character(self, data: Dict[str, Any]) -> None:
        """Store a character description."""
        try:
            character = Character.from_dict(data)
        except Exception:  # pragma: no cover - best effort
            return
        self.characters.add(character)
        self.characters.save()

    def _update_world(self, data: Dict[str, Any]) -> None:
        """Store world information using the legacy ``add`` API."""
        name = data.get("name")
        info = data.get("info", {})
        if not name:
            return
        self.worlds.add(name, info)
        self.worlds.save()

    def _update_style(self, data: Dict[str, Any]) -> None:
        """Store style preferences or examples."""
        user_id = data.get("user_id", "default")
        author = data.get("author", "preferred")
        example = data.get("example")
        description = data.get("description")
        characteristics = data.get("characteristics")
        self.styles.add(
            user_id,
            author,
            example=example,
            description=description,
            characteristics=characteristics,
        )
        self.styles.save()


__all__ = ["FeedbackLearner"]
