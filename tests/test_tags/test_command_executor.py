import pytest

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
