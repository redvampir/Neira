from datetime import datetime, timedelta

from chat.memory_store import MemoryStore


def test_tree_structure_and_export_import(tmp_path):
    store = MemoryStore()
    parent = store.add_note("parent", tags=["root"])
    child = store.add_note("child", parent_id=parent.id, tags=["child"])
    store.add_note("grandchild", parent_id=child.id, tags=["grand"])

    path = tmp_path / "session.neira-chat"
    store.export_session(path)

    restored = MemoryStore()
    restored.import_session(path)

    assert restored.root.children[0].content == "parent"
    assert restored.root.children[0].children[0].content == "child"

    notes = restored.find_by_tags(["grand"])
    assert notes[0].content == "grandchild"

    start = datetime.utcnow() - timedelta(days=1)
    assert restored.find_by_date(start, None)
