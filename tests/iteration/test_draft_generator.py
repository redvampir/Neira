from src.iteration import DraftGenerator
from src.memory import CharacterMemory, WorldMemory, StyleMemory, MemoryIndex
from src.models import Character


def test_returns_from_hot_memory(tmp_path):
    index = MemoryIndex()
    index.set("question", "answer")

    generator = DraftGenerator(
        CharacterMemory(storage_path=tmp_path / "chars.json"),
        WorldMemory(storage_path=tmp_path / "world.json"),
        StyleMemory(storage_path=tmp_path / "style.json"),
    )

    result = generator.generate_draft("question", index)
    assert result == "answer"


def test_fallback_to_character_memory(tmp_path):
    char_mem = CharacterMemory(storage_path=tmp_path / "chars.json")
    char_mem.add(Character(name="Alice", appearance="brave"))

    generator = DraftGenerator(
        char_mem,
        WorldMemory(storage_path=tmp_path / "world.json"),
        StyleMemory(storage_path=tmp_path / "style.json"),
    )

    index = MemoryIndex()
    result = generator.generate_draft("Alice", index)
    assert "Alice" in result


def test_style_memory_fallback(tmp_path):
    style_mem = StyleMemory(storage_path=tmp_path / "style.json")
    style_mem.add("default", "classic", example="stylish example")

    generator = DraftGenerator(
        CharacterMemory(storage_path=tmp_path / "chars.json"),
        WorldMemory(storage_path=tmp_path / "world.json"),
        style_mem,
    )

    index = MemoryIndex()
    result = generator.generate_draft("unknown", index)
    assert result == "stylish example"
