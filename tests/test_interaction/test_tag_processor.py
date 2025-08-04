"""Tests for the advanced tag processor."""

from __future__ import annotations

import json
from pathlib import Path

from src.interaction.tag_processor import TagProcessor
from src.memory.knowledge_base import analyze_book


def _prepare_kb(tmp_path: Path) -> None:
    text = "Глава 1\nЛили смотрела на море.\n"
    file = tmp_path / "book.txt"
    file.write_text(text, encoding="utf-8")
    analyze_book(str(file))


def test_parse_character_tag() -> None:
    processor = TagProcessor()
    tag = processor.parse("@Персонаж: Лили — внешность@")[0]
    assert tag.subject == "Лили"
    assert tag.commands == ["внешность"]


def test_parse_with_slash_commands() -> None:
    processor = TagProcessor()
    tag = processor.parse("@Персонаж: Лили /внешность /стиль@")[0]
    assert tag.commands == ["внешность", "стиль"]


def test_suggest_entities(tmp_path) -> None:
    _prepare_kb(tmp_path)
    processor = TagProcessor()
    suggestions = processor.suggest_entities("Л")
    assert "Лили" in suggestions


def test_extract_style_examples(tmp_path) -> None:
    processor = TagProcessor()
    text = "[Пример стиля автора, из главы 1]\nТишина.\n[Пример окончен]"
    examples = processor.extract_style_examples(text)
    assert examples == ["Тишина."]
    data = json.loads((Path("data/knowledge_base") / "style.json").read_text(encoding="utf-8"))
    assert "Тишина." in data["examples"]


def test_execute_generation_without_llm() -> None:
    processor = TagProcessor()
    tag = processor.parse("@Сцена: лесное приключение /сгенерировать@")[0]
    output = processor.execute(tag)
    assert "Сгенерируй сцену: лесное приключение" in output

