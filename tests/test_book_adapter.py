# tests/test_book_adapter.py
"""Tests for book extraction and adaptation utilities."""

from integration.books.extractor import SettingExtractor
from integration.books.adapter import BookToRPGAdapter


def test_setting_extractor_basic_functions():
    content = (
        "The city Neverwinter must remain hidden. "
        "Knights shall protect the realm. "
        "A mysterious quest awaits in the forest Greenwood. "
        "There is a secret threat rising."
    )
    characters = [{"name": "Alice", "traits": ["brave"]}, "Bob"]

    extractor = SettingExtractor()

    rules = extractor.extract_world_rules(content)
    assert "The city Neverwinter must remain hidden" in rules
    assert "Knights shall protect the realm" in rules

    npcs = extractor.create_npcs_from_characters(characters)
    assert any(npc["name"] == "Alice" for npc in npcs)
    assert any(npc["name"] == "Bob" for npc in npcs)

    locations = extractor.generate_locations(content)
    assert "Neverwinter" in locations
    assert "Greenwood" in locations

    hooks = extractor.extract_plot_hooks(content)
    assert any("mysterious quest" in hook for hook in hooks)
    assert any("secret threat" in hook for hook in hooks)


def test_book_to_rpg_adapter_workflow():
    content = (
        "The city Neverwinter must remain hidden. "
        "A mysterious quest awaits in the forest Greenwood."
    )
    book = {
        "title": "Mystery of Neverwinter",
        "content": content,
        "characters": [{"name": "Alice", "level": 2}, {"name": "Bob", "level": 4}],
    }
    adapter = BookToRPGAdapter()
    campaign = adapter.create_campaign_from_book(book, "DND")

    assert campaign["title"] == "Mystery of Neverwinter"
    assert campaign["system"] == "DND"
    assert "Neverwinter" in campaign["locations"]
    assert campaign["npcs"][0]["name"] == "Alice"

    balanced = adapter.balance_characters_for_game(book["characters"])
    assert all(char["level"] == 3 for char in balanced)

    adventures = adapter.generate_adventures(campaign["plot_hooks"])
    assert adventures
    assert adventures[0]["summary"] in campaign["plot_hooks"]
