from __future__ import annotations

from pathlib import Path

from src.memory.lazy_loader import LazyMemoryLoader


def create_book(tmp_path: Path) -> Path:
    book_dir = tmp_path / "book"
    book_dir.mkdir()
    for i in range(1, 4):
        (book_dir / f"chapter{i}.txt").write_text(f"content {i}", encoding="utf-8")
    return book_dir


def test_loads_index_without_content(tmp_path: Path) -> None:
    book_dir = create_book(tmp_path)
    loader = LazyMemoryLoader(book_dir)

    index = loader.load_book_index()
    assert set(index.keys()) == {"chapter1", "chapter2", "chapter3"}
    assert loader._cache == {}


def test_get_book_chapter_caches_and_evicts(tmp_path: Path) -> None:
    book_dir = create_book(tmp_path)
    loader = LazyMemoryLoader(book_dir, max_cache_size=1)
    loader.load_book_index()

    assert loader.get_book_chapter("chapter1") == "content 1"
    assert "chapter1" in loader._cache

    loader.get_book_chapter("chapter2")
    assert "chapter2" in loader._cache
    assert "chapter1" not in loader._cache


def test_clear_cache(tmp_path: Path) -> None:
    book_dir = create_book(tmp_path)
    loader = LazyMemoryLoader(book_dir)
    loader.load_book_index()

    loader.get_book_chapter("chapter1")
    assert loader._cache
    loader.clear_cache()
    assert loader._cache == {}
