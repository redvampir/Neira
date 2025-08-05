"""Persist and retrieve session information for the application."""

from __future__ import annotations

from dataclasses import dataclass
from datetime import datetime
from typing import Any, Dict, List


@dataclass
class SessionRecord:
    """Internal record capturing the session state."""

    session: Any
    saved_at: datetime


class SessionMemory:
    """In-memory storage for session states and character progression."""

    def __init__(self) -> None:
        self._sessions: Dict[str, SessionRecord] = {}
        self._development: Dict[str, List[Dict[str, Any]]] = {}

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

    def load_session_state(self, session_id: str) -> Any | None:
        """Return previously saved session for ``session_id`` if present."""
        record = self._sessions.get(session_id)
        return record.session if record else None

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
