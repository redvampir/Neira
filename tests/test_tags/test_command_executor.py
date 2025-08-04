import pytest
from unittest.mock import patch

from src.tags.command_executor import CommandExecutor
from src.tags.tag_parser import Tag


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
    class Dummy:
        def __init__(self):
            self.emotional_state = "neutral"
            self.characters_memory = {}

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
