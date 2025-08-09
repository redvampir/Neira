"""Simple in-memory database for museum items.

This module provides a small utility used in the tests for this kata.  The
database stores items and allows them to be retrieved by tags.  It also exposes
helpers for exporting collections of items into ``.neira-pack`` files – a JSON
based interchange format used by the project.

The implementation intentionally avoids any external dependencies and keeps the
data in memory.  For a real application the persistence layer would be far more
advanced, however the current functionality is perfectly adequate for unit
tests and examples.
"""

from __future__ import annotations

from dataclasses import dataclass, asdict, field
from pathlib import Path
from typing import Dict, Iterable, List, Set
import json


@dataclass
class Item:
    """Representation of an item stored in the database.

    Parameters
    ----------
    id:
        Identifier of the item.
    data:
        Arbitrary payload associated with the item.  Tests use a mapping with a
        ``name`` key but the structure is intentionally flexible.
    tags:
        Set of tags describing the item.  Tags are indexed by the database and
        allow quick lookup of items belonging to a particular group.
    votes / score:
        Used by :class:`museum.ui.rating_panel.RatingPanel` to keep track of user
        votes.  ``score`` is the sum of all vote values whereas ``votes`` stores
        the amount of votes cast for the item.  The ``rating`` property exposes
        the average score.
    """

    id: str
    data: Dict[str, object]
    tags: Set[str] = field(default_factory=set)
    votes: int = 0
    score: int = 0

    @property
    def rating(self) -> float:
        """Average rating for the item."""

        if not self.votes:
            return 0.0
        return self.score / self.votes

    def to_dict(self) -> Dict[str, object]:
        """Serialise the item into a JSON compatible mapping."""

        data = asdict(self)
        # ``tags`` is a set in memory but JSON requires a list.  Sorting keeps
        # the output deterministic which simplifies testing.
        data["tags"] = sorted(self.tags)
        data["rating"] = self.rating
        return data


class Database:
    """In-memory storage for :class:`Item` instances."""

    def __init__(self) -> None:
        self._items: Dict[str, Item] = {}
        self._tag_index: Dict[str, Set[str]] = {}

    # ------------------------------------------------------------------
    # item management
    # ------------------------------------------------------------------
    def add_item(self, item: Item) -> None:
        """Add ``item`` to the database and index it by tags."""

        self._items[item.id] = item
        for tag in item.tags:
            self._tag_index.setdefault(tag, set()).add(item.id)

    def get_item(self, item_id: str) -> Item:
        """Return the item with ``item_id``.

        ``KeyError`` is propagated if the item is unknown which helps tests
        identify mistakes quickly.
        """

        return self._items[item_id]

    def find_by_tag(self, tag: str) -> List[Item]:
        """Return all items associated with ``tag``.

        The items are returned in deterministic order based on their identifier
        to make unit testing easier.
        """

        ids = sorted(self._tag_index.get(tag, set()))
        return [self._items[i] for i in ids]

    # ------------------------------------------------------------------
    # export helpers
    # ------------------------------------------------------------------
    def export_collection(
        self, item_ids: Iterable[str], path: Path | str
    ) -> None:
        """Export ``item_ids`` to ``path``.

        The file format is JSON but uses the ``.neira-pack`` extension.  Each
        item is serialised using :meth:`Item.to_dict`.
        """

        serialised = [self.get_item(i).to_dict() for i in item_ids]
        out_path = Path(path)
        out_path.write_text(
            json.dumps(serialised, ensure_ascii=False, indent=2),
            encoding="utf-8",
        )


__all__ = ["Database", "Item"]
