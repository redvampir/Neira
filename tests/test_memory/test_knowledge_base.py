"""Tests for the knowledge base builder."""

from __future__ import annotations

import json
from pathlib import Path

import pytest

from src.memory.knowledge_base import analyze_book


def test_analyze_book_creates_files(tmp_path) -> None:
    book = (
        "Глава 1\n"
        "Лили пошла в Лес.\n"
        "[Пример стиля автора, из главы 1]\n"
        "Стильный отрывок.\n"
        "[Пример окончен]\n"
        "Глава 2\n"
        "Вилл встретил Лили.\n"
    )
    file = tmp_path / "book.txt"
    file.write_text(book, encoding="utf-8")

    result = analyze_book(str(file))

    kb_dir = Path("data/knowledge_base")
    characters_path = kb_dir / "characters.json"
    style_path = kb_dir / "style.json"

    assert characters_path.exists()
    assert style_path.exists()

    characters = json.loads(characters_path.read_text(encoding="utf-8"))
    assert "лили" in characters
    assert "Вилл" in result["index"]["characters"]

    style = json.loads(style_path.read_text(encoding="utf-8"))
    assert style["examples"]


def test_analyze_book_missing_file(tmp_path: Path) -> None:
    missing = tmp_path / "missing.txt"
    with pytest.raises(FileNotFoundError) as exc:
        analyze_book(str(missing))
    assert "not found" in str(exc.value).lower()

