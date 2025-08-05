import json
import pytest
from unittest.mock import patch

from src.memory import CharacterMemory, StyleMemory
from src.tags.command_executor import CommandExecutor
from src.tags.enhanced_parser import Tag


def test_unknown_tag_returns_message():
    executor = CommandExecutor()
    tag = Tag(type='unknown', content='something', position=(0, 0))
    result = executor.execute_command(tag)
    assert "учусь" in result


def test_register_custom_handler():
    executor = CommandExecutor()

    def handler(content: str, context):
        return f"custom:{content}"

    executor.register_handler('custom', handler)
    tag = Tag(type='custom', content='data', position=(0, 0))
    assert executor.execute_command(tag) == "custom:data"


def test_emotion_updates_brain(tmp_path):
    class Dummy:
        def __init__(self):
            self.emotional_state = "neutral"
            self.characters_memory = CharacterMemory(tmp_path / "chars.json")

    brain = Dummy()
    executor = CommandExecutor(brain)
    tag = Tag(type='emotion_paint', content='грусть', position=(0, 0))
    executor.execute_command(tag)
    assert brain.emotional_state == 'грусть'


def test_description_handler_without_llm():
    executor = CommandExecutor()
    tag = Tag(type='description_write', content='тихий лес', position=(0, 0))
    result = executor.execute_command(tag)
    assert 'Описание' in result


def test_create_smart_dialogue_without_llm_selects_default_template():
    executor = CommandExecutor()
    context = {"characters": ["Алиса", "Боб"]}
    with patch("src.tags.command_executor.random.choice", side_effect=lambda seq: seq[0]):
        result = executor._create_smart_dialogue("Привет", context)

    assert "Слушай" in result
    assert "Точно!" in result
    assert "Стиль: дружеский" in result


def test_style_example_handler_persists_example(tmp_path):
    class Dummy:
        def __init__(self) -> None:
            self.style_memory = StyleMemory(tmp_path / "styles.json")

    brain = Dummy()
    executor = CommandExecutor(brain)
    tag = Tag(
        type='style_example',
        content='пример',
        position=(0, 0),
        params={'author': 'Автор'}
    )
    executor.execute_command(tag)
    data = json.loads((tmp_path / "styles.json").read_text(encoding="utf-8"))
    assert 'пример' in data['Автор']['examples']


def test_character_reminder_handler_updates_memory(tmp_path):
    class Dummy:
        def __init__(self) -> None:
            self.characters_memory = CharacterMemory(tmp_path / "chars.json")

    brain = Dummy()
    executor = CommandExecutor(brain)
    tag = Tag(
        type='character_reminder',
        content='',
        position=(0, 0),
        params={'name': 'Лили', 'appearance': 'светлые волосы', 'traits': 'добрая'}
    )
    executor.execute_command(tag)
    data = json.loads((tmp_path / "chars.json").read_text(encoding="utf-8"))
    assert data['Лили']['appearance'] == 'светлые волосы'
    assert 'добрая' in data['Лили']['personality_traits']


def test_generate_content_handler_without_llm():
    executor = CommandExecutor()
    tag = Tag(type='generate_content', content='история', position=(0, 0))
    result = executor.execute_command(tag)
    assert 'история' in result
