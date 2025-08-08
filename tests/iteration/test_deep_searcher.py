from src.iteration import DeepSearcher
from src.memory import CharacterMemory, WorldMemory, StyleMemory, MemoryIndex
from src.models import Character
from src.search import SearchAPIClient


def test_search_aggregates_sources_with_priorities(tmp_path):
    char_mem = CharacterMemory(storage_path=tmp_path / "chars.json")
    world_mem = WorldMemory(storage_path=tmp_path / "world.json")
    world_mem.add_rule("Fantasy", "magic", "Dragons are common")
    style_mem = StyleMemory(storage_path=tmp_path / "style.json")
    style_mem.add("u1", "Bob", description="whimsical tales about dragons")

    cold_file = tmp_path / "cold.txt"
    cold_file.write_text("Ancient dragons slumber.", encoding="utf-8")

    def fake_fetch(query: str, limit: int):
        return [{"url": "https://example.com", "snippet": f"{query} on web"}]

    api = SearchAPIClient(memory=MemoryIndex(), fetcher=fake_fetch)

    searcher = DeepSearcher(
        char_mem,
        world_mem,
        style_mem,
        api_client=api,
        data_path=tmp_path,
    )

    results = searcher.search("dragon", user_id="u1")

    assert [r["source"] for r in results] == ["world_memory", "style_memory", "file", "web"]
    assert results[2]["reference"] == str(cold_file)


def test_character_memory_search(tmp_path):
    char_mem = CharacterMemory(storage_path=tmp_path / "chars.json")
    char_mem.add(Character(name="Alice", appearance="brave"))

    def fake_fetch(query: str, limit: int):
        return [{"url": "https://example.com", "snippet": f"{query}"}]

    searcher = DeepSearcher(
        char_mem,
        WorldMemory(storage_path=tmp_path / "world.json"),
        StyleMemory(storage_path=tmp_path / "style.json"),
        api_client=SearchAPIClient(memory=MemoryIndex(), fetcher=fake_fetch),
        data_path=tmp_path,
    )

    results = searcher.search("Alice", user_id="u1")

    assert results[0]["source"] == "character_memory"
    assert results[0]["content"]["name"] == "Alice"
