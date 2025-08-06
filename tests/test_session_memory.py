from __future__ import annotations

import json
import time
from dataclasses import dataclass

from src.core.session_memory import SessionMemory


@dataclass
class DummySession:
    id: str
    value: int


def test_persistence(tmp_path) -> None:
    path = tmp_path / "sessions.json"
    memory = SessionMemory(storage_path=str(path))
    memory.save_session_state(DummySession(id="s1", value=42))

    assert path.exists()
    with open(path, "r", encoding="utf-8") as fh:
        data = json.load(fh)
    assert "s1" in data
    assert "saved_at" in data["s1"]

    reloaded = SessionMemory(storage_path=str(path))
    loaded = reloaded.load_session_state("s1")
    assert isinstance(loaded, dict)
    assert loaded["value"] == 42


def test_expiration(tmp_path) -> None:
    path = tmp_path / "sessions.json"
    memory = SessionMemory(storage_path=str(path), ttl_seconds=1)
    memory.save_session_state(DummySession(id="s1", value=42))
    time.sleep(1.1)
    reloaded = SessionMemory(storage_path=str(path), ttl_seconds=1)
    assert reloaded.load_session_state("s1") is None
