"""Проверка личности Нейры"""

from src.core.neyra_config import NeyraPersonality


def test_personality_defaults() -> None:
    """Убеждаюсь, что черты личности имеют ожидаемые значения"""
    personality = NeyraPersonality()
    assert 0 <= personality.curiosity_level <= 1
