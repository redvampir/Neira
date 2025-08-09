from pathlib import Path
import json
import sqlite3
import sys

sys.path.append(str(Path(__file__).resolve().parents[2]))
from modes.resource_manager import ResourceManagerMode


def test_import_and_search(tmp_path: Path) -> None:
    root = tmp_path / "resources"
    manager = ResourceManagerMode(root=root)

    # create external file to import
    src = tmp_path / "sample.txt"
    src.write_text("hello", encoding="utf-8")

    manager.import_resource(src, tags=["text", "sample"], metadata={"size": 5})

    all_resources = manager.list_resources()
    assert len(all_resources) == 1
    assert all_resources[0].tags == ["text", "sample"]

    results = manager.search("sample")
    assert results and results[0].name == "sample.txt"

    # verify raw DB contents for tags and metadata
    conn = sqlite3.connect(root / "index.db")
    row = conn.execute("SELECT tags, metadata FROM resources").fetchone()
    assert row is not None
    assert row[0] == "text,sample"
    assert json.loads(row[1])["size"] == 5
