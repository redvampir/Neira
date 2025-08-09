"""Voting UI helper for museum items.

The real project would expose a graphical or web based interface.  For the
purposes of the tests the :class:`RatingPanel` class merely provides a thin
wrapper around :class:`museum.database.Database` that records votes and computes
an average rating for each item.
"""

from __future__ import annotations

from dataclasses import dataclass

from ..database import Database


@dataclass
class RatingPanel:
    """Simple interface used in tests to cast votes for items."""

    db: Database

    def vote(self, item_id: str, value: int) -> float:
        """Register a vote for ``item_id``.

        ``value`` should be ``1`` for an up‑vote and ``-1`` for a down‑vote.  The
        method returns the new average rating of the item.
        """

        if value not in (1, -1):  # pragma: no cover - defensive check
            raise ValueError("vote value must be 1 or -1")

        item = self.db.get_item(item_id)
        item.votes += 1
        item.score += value
        return item.rating

    def get_rating(self, item_id: str) -> float:
        """Return the current average rating for ``item_id``."""

        return self.db.get_item(item_id).rating

    def render(self, item_id: str) -> dict:
        """Return data representing the panel state.

        The return value is a serialisable mapping which can be passed to a
        front‑end.  Keeping the method lightweight simplifies testing and keeps
        the component framework agnostic.
        """

        item = self.db.get_item(item_id)
        return {"item_id": item_id, "rating": item.rating, "votes": item.votes}


__all__ = ["RatingPanel"]
