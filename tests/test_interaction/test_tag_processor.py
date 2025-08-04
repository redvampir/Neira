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


def test_parse_with_escaped_at() -> None:
    processor = TagProcessor()
    tag = processor.parse("@Персонаж: Лили@@")[0]
    assert tag.subject == "Лили@"


def test_parse_nested_tags() -> None:
    processor = TagProcessor()
    text = "@Сцена: встреча с @Персонаж: Лили@@@ в кафе@"
    tags = processor.parse(text)
    assert tags[0].type == "персонаж" and tags[0].subject == "Лили@"
    assert tags[1].type == "сцена" and tags[1].subject == "встреча с @Персонаж: Лили@ в кафе"


def test_suggest_entities(tmp_path) -> None:
    _prepare_kb(tmp_path)
    processor = TagProcessor()
    suggestions = processor.suggest_entities("Л")
    assert "Лили" in suggestions


def test_generate_hints(tmp_path) -> None:
    _prepare_kb(tmp_path)
    processor = TagProcessor()
    hints = processor.generate_hints("Л")
    assert "Лили" in hints


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


def test_execute_slash_generation() -> None:
    processor = TagProcessor()
    output = processor.execute_slash("/сгенерировать лес")
    assert "Сгенерируй сцену: лес" in output

