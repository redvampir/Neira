import pytest
from unittest.mock import patch
from typing import Any, Dict

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


def test_emotion_updates_brain():
    from src.memory import CharacterMemory

    class Dummy:
        def __init__(self):
            self.emotional_state = "neutral"
            self.characters_memory = CharacterMemory()

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


def test_style_example_handler_appends_to_context():
    executor = CommandExecutor()
    tag = Tag(type='style_example', content='пример', position=(0, 0))
    ctx: Dict[str, Any] = {}
    result = executor.execute_command(tag, ctx)
    assert 'пример' in ctx.get('style_examples', [])
    assert 'пример' in result


def test_character_reminder_handler():
    executor = CommandExecutor()
    tag = Tag(type='character_reminder', content='Лили', position=(0, 0))
    result = executor.execute_command(tag)
    assert 'Лили' in result


def test_generate_content_handler_without_llm():
    executor = CommandExecutor()
    tag = Tag(type='generate_content', content='история', position=(0, 0))
    result = executor.execute_command(tag)
    assert 'история' in result
