from pathlib import Path
import json

from museum.database import Database, Item


def test_store_and_query_tags() -> None:
    db = Database()
    item1 = Item(id="1", data={"name": "Mona Lisa"}, tags={"art", "renaissance"})
    item2 = Item(id="2", data={"name": "Starry Night"}, tags={"art"})
    db.add_item(item1)
    db.add_item(item2)

    art_items = db.find_by_tag("art")
    assert art_items == [item1, item2]
    renaissance_items = db.find_by_tag("renaissance")
    assert renaissance_items == [item1]


def test_export_collection(tmp_path: Path) -> None:
    db = Database()
    item = Item(id="1", data={"name": "Mona Lisa"}, tags={"art"})
    db.add_item(item)

    out_file = tmp_path / "collection.neira-pack"
    db.export_collection(["1"], out_file)

    loaded = json.loads(out_file.read_text(encoding="utf-8"))
    assert loaded[0]["id"] == "1"
    assert loaded[0]["tags"] == ["art"]
