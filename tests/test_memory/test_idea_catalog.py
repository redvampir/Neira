from __future__ import annotations

import json
from pathlib import Path

from src.memory.idea_catalog import IdeaCatalog


def test_crud_operations(tmp_path: Path) -> None:
    storage = tmp_path / "catalog.json"
    catalog = IdeaCatalog(storage)

    # Starts empty
    assert catalog.get() == {}

    # Add entry and ensure it's saved
    catalog.add("foo", "bar")
    assert catalog.get("foo") == "bar"
    assert json.loads(storage.read_text()) == {"foo": "bar"}

    # Update and persist
    catalog.update("foo", "baz")
    assert catalog.get("foo") == "baz"
    assert json.loads(storage.read_text()) == {"foo": "baz"}

    # Delete and persist
    catalog.delete("foo")
    assert catalog.get() == {}
    assert json.loads(storage.read_text()) == {}
