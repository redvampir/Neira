"""Persist and retrieve session information for the application."""

from __future__ import annotations

from dataclasses import dataclass, asdict, is_dataclass
from datetime import datetime, timedelta
import json
import os
from typing import Any, Dict, List


@dataclass
class SessionRecord:
    """Internal record capturing the session state."""

    session: Any
    saved_at: datetime


class SessionMemory:
    """In-memory storage for session states and character progression."""

    def __init__(
        self,
        storage_path: str | None = None,
        ttl_seconds: int | None = None,
    ) -> None:
        self.storage_path = storage_path
        self.ttl_seconds = ttl_seconds
        self._sessions: Dict[str, SessionRecord] = {}
        self._development: Dict[str, List[Dict[str, Any]]] = {}
        if self.storage_path:
            self.load_from_disk()

    def save_session_state(self, session: Any) -> None:
        """Store ``session`` by its ``id`` attribute."""
        session_id = getattr(session, "id", None) or getattr(
            session, "session_id", None
        )
        if session_id is None:
            raise ValueError("session must have an 'id' or 'session_id' attribute")
        self._sessions[session_id] = SessionRecord(
            session=session, saved_at=datetime.utcnow()
        )
        self.save_to_disk()

    def load_session_state(self, session_id: str) -> Any | None:
        """Return previously saved session for ``session_id`` if present."""
        record = self._sessions.get(session_id)
        return record.session if record else None

    # ------------------------------------------------------------------
    # Persistence helpers

    def _serialize_session(self, session: Any) -> Any:
        if hasattr(session, "to_dict") and callable(getattr(session, "to_dict")):
            return session.to_dict()
        if is_dataclass(session):
            return asdict(session)
        if hasattr(session, "__dict__"):
            return session.__dict__
        return session

    def save_to_disk(self) -> None:
        """Persist ``_sessions`` to ``storage_path`` if configured."""
        if not self.storage_path:
            return
        os.makedirs(os.path.dirname(self.storage_path), exist_ok=True)
        data = {
            sid: {
                "session": self._serialize_session(rec.session),
                "saved_at": rec.saved_at.isoformat(),
            }
            for sid, rec in self._sessions.items()
        }
        with open(self.storage_path, "w", encoding="utf-8") as fh:
            json.dump(data, fh)

    def load_from_disk(self) -> None:
        """Load sessions from ``storage_path``, purging entries beyond TTL."""
        if not self.storage_path or not os.path.exists(self.storage_path):
            return
        with open(self.storage_path, "r", encoding="utf-8") as fh:
            raw: Dict[str, Dict[str, Any]] = json.load(fh)
        now = datetime.utcnow()
        ttl = (
            timedelta(seconds=self.ttl_seconds)
            if self.ttl_seconds is not None
            else None
        )
        self._sessions = {}
        changed = False
        for sid, rec in raw.items():
            saved_at = datetime.fromisoformat(rec["saved_at"])
            if ttl and now - saved_at > ttl:
                changed = True
                continue
            self._sessions[sid] = SessionRecord(
                session=rec["session"],
                saved_at=saved_at,
            )
        if changed:
            self.save_to_disk()

    def track_character_development(self, character: Any) -> List[Dict[str, Any]]:
        """Append a timestamped snapshot of ``character`` state."""
        character_id = getattr(character, "id", None)
        if character_id is None:
            raise ValueError("character must have an 'id' attribute")
        snapshot = {
            "timestamp": datetime.utcnow(),
            "state": getattr(character, "state", None),
        }
        history = self._development.setdefault(character_id, [])
        history.append(snapshot)
        return list(history)
