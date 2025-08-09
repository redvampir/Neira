from datetime import datetime, timedelta

from chat.memory_store import MemoryStore
from chat.ui.filter_panel import FilterPanel


def test_filter_panel_tags_and_dates():
    store = MemoryStore()
    store.add_note("old", tags=["x"], created=datetime.utcnow() - timedelta(days=5))
    newer = store.add_note("new", tags=["x", "y"], created=datetime.utcnow())

    panel = FilterPanel()
    panel.set_filters(tags=["y"])
    notes = panel.apply(store.find_by_tags(["x"]))
    assert notes == [newer]

    panel.set_filters(start=datetime.utcnow() - timedelta(days=1))
    notes = panel.apply(store.find_by_tags(["x"]))
    assert notes == [newer]
