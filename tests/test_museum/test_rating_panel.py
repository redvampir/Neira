from museum.database import Database, Item
from museum.ui.rating_panel import RatingPanel


def test_rating_panel_records_votes() -> None:
    db = Database()
    item = Item(id="1", data={"name": "Mona Lisa"})
    db.add_item(item)
    panel = RatingPanel(db)

    assert panel.vote("1", 1) == 1.0
    assert panel.vote("1", -1) == 0.0
    assert panel.get_rating("1") == 0.0
    assert item.votes == 2
