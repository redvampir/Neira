from __future__ import annotations

"""Simple hierarchical note storage with session export/import."""

from dataclasses import dataclass, field, asdict
from datetime import datetime
import json
from pathlib import Path
from typing import Dict, Iterable, List, Optional


@dataclass
class Note:
    """Single note item in the tree."""

    id: int
    content: str
    tags: List[str] = field(default_factory=list)
    created: datetime = field(default_factory=datetime.utcnow)
    children: List["Note"] = field(default_factory=list)

    def to_dict(self) -> dict:
        """Return a JSON serialisable representation of the note."""

        data = asdict(self)
        data["created"] = self.created.isoformat()
        data["children"] = [c.to_dict() for c in self.children]
        return data


class MemoryStore:
    """Manage hierarchical notes and session persistence."""

    def __init__(self) -> None:
        self.root = Note(id=0, content="root")
        self._notes: Dict[int, Note] = {0: self.root}
        self._next_id = 1

    # ------------------------------------------------------------------
    def add_note(
        self,
        content: str,
        parent_id: int | None = None,
        tags: Optional[Iterable[str]] = None,
        created: Optional[datetime] = None,
    ) -> Note:
        """Create a new note under ``parent_id`` and return it."""

        if parent_id is None:
            parent = self.root
        else:
            parent = self._notes.get(parent_id)
            if parent is None:
                raise KeyError(f"unknown parent_id {parent_id}")
        note = Note(
            id=self._next_id,
            content=content,
            tags=list(tags or []),
            created=created or datetime.utcnow(),
        )
        parent.children.append(note)
        self._notes[note.id] = note
        self._next_id += 1
        return note

    # ------------------------------------------------------------------
    def find_by_tags(self, tags: Iterable[str]) -> List[Note]:
        """Return all notes containing *all* ``tags``."""

        wanted = set(tags)
        return [n for n in self._notes.values() if wanted.issubset(n.tags)]

    def find_by_date(self, start: datetime | None, end: datetime | None) -> List[Note]:
        """Return notes created within ``start``/``end`` interval."""

        result: List[Note] = []
        for note in self._notes.values():
            if start and note.created < start:
                continue
            if end and note.created > end:
                continue
            result.append(note)
        return result

    # ------------------------------------------------------------------
    def export_session(self, path: str | Path) -> None:
        """Export the whole note tree to ``path``.

        ``path`` must have the ``.neira-chat`` extension.
        """

        p = Path(path)
        if p.suffix != ".neira-chat":
            raise ValueError("Session files must use .neira-chat extension")
        data = self.root.to_dict()
        p.write_text(json.dumps(data, ensure_ascii=False, indent=2), encoding="utf-8")

    def import_session(self, path: str | Path) -> None:
        """Load a session from ``path`` overwriting current notes."""

        p = Path(path)
        if p.suffix != ".neira-chat":
            raise ValueError("Session files must use .neira-chat extension")
        data = json.loads(p.read_text(encoding="utf-8"))
        self.root = self._dict_to_note(data)
        self._notes = {}
        self._reindex(self.root)

    # ------------------------------------------------------------------
    def _dict_to_note(self, data: dict) -> Note:
        children = [self._dict_to_note(d) for d in data.get("children", [])]
        note = Note(
            id=data["id"],
            content=data["content"],
            tags=list(data.get("tags", [])),
            created=datetime.fromisoformat(data["created"]),
            children=children,
        )
        return note

    def _reindex(self, note: Note) -> None:
        self._notes[note.id] = note
        self._next_id = max(self._next_id, note.id + 1)
        for child in note.children:
            self._reindex(child)

