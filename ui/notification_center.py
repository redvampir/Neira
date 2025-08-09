from __future__ import annotations

from dataclasses import dataclass
from datetime import datetime
from pathlib import Path
from typing import List


@dataclass
class Notification:
    timestamp: datetime
    level: str
    message: str
    source: str


_notifications: List[Notification] = []

IMPORTANT_LEVELS = {"warning", "error", "critical"}
LOG_FILE = Path("userdata/notifications.log")


if LOG_FILE.exists():
    try:
        with LOG_FILE.open("r", encoding="utf-8") as f:
            for line in f:
                ts, level, source, message = line.rstrip("\n").split("\t", 3)
                _notifications.append(
                    Notification(datetime.fromisoformat(ts), level, message, source)
                )
    except Exception:
        # If the log is malformed we simply start with an empty list
        _notifications.clear()


def _persist(note: Notification) -> None:
    LOG_FILE.parent.mkdir(parents=True, exist_ok=True)
    with LOG_FILE.open("a", encoding="utf-8") as f:
        f.write(
            f"{note.timestamp.isoformat()}\t{note.level}\t{note.source}\t{note.message}\n"
        )


def notify(level: str, message: str, source: str) -> None:
    """Record a notification and persist important ones."""
    note = Notification(datetime.utcnow(), level, message, source)
    _notifications.append(note)
    if level.lower() in IMPORTANT_LEVELS:
        _persist(note)


def list_notifications() -> List[Notification]:
    """Return recorded notifications."""
    return list(_notifications)
