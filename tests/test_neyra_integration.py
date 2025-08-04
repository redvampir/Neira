# tests/test_neyra_integration.py
"""Интеграционные тесты для проверки работы Нейры в целом"""

import pytest
from pathlib import Path
from src.core.neyra_brain import Neyra
from src.memory import CharacterMemory


def test_neyra_awakening():
    """Проверяю, что Нейра правильно просыпается"""
    neyra = Neyra()
    assert neyra.emotional_state == "любопытная"
    assert len(neyra.known_books) == 0
    assert isinstance(neyra.characters_memory, CharacterMemory)


def test_neyra_understands_basic_tags():
    """Нейра должна понимать основные теги"""
    neyra = Neyra()
    
    command = "@Нейра: Создай романтическую сцену@ @Эмоция: нежность@"
    result = neyra.process_command(command)
    
    assert "сцену" in result or "Создаю" in result
    assert "нежность" in result


def test_neyra_remembers_characters():
    """Нейра должна запоминать персонажей"""
    neyra = Neyra()
    
    command = "@Персонаж: Алиса - смелая девочка@"
    result = neyra.process_command(command)
    
    assert "Алиса" in neyra.characters_memory
    assert "Алиса" in result


def test_neyra_loads_books(tmp_path):
    """Нейра должна загружать и анализировать книги"""
    neyra = Neyra()
    
    # Создаю тестовую книгу
    test_book = tmp_path / "test_story.txt"
    test_content = """
    Алиса шла по лесу. Вдруг она встретила Белого Кролика.
    "Опаздываю, опаздываю!" - закричал Кролик и побежал дальше.
    Алиса последовала за ним в кроличью нору.
    """
    test_book.write_text(test_content, encoding='utf-8')
    
    # Загружаю книгу
    neyra.load_book(str(test_book))
    
    assert str(test_book) in neyra.known_books
    assert "Алиса" in neyra.characters_memory
    assert "Кролик" in neyra.characters_memory or "Белого" in neyra.characters_memory


def test_neyra_creative_responses():
    """Нейра должна давать творческие ответы"""
    neyra = Neyra()
    
    # Тест создания сцены
    scene_command = "@Нейра: Создай загадочную сцену@"
    scene_result = neyra.process_command(scene_command)
    
    assert len(scene_result) > 50  # Должен быть развернутый ответ
    assert "сцену" in scene_result.lower()
    
    # Тест работы с эмоциями
    emotion_command = "@Эмоция: грусть@"
    emotion_result = neyra.process_command(emotion_command)
    
    assert "грусть" in emotion_result
    assert neyra.emotional_state == "грусть"


def test_neyra_handles_complex_commands():
    """Нейра должна обрабатывать сложные команды с несколькими тегами"""
    neyra = Neyra()
    
    complex_command = (
        "@Нейра: Создай диалог@ "
        "@Персонаж: Элизабет - волнуется@ "
        "@Эмоция: нервозность@ "
        "@Стиль: формальный@"
    )
    
    result = neyra.process_command(complex_command)
    
    # Должна обработать все теги
    assert "Элизабет" in result
    assert "нервозность" in result
    assert "формальный" in result


def test_neyra_recall_history():
    """Нейра должна вспоминать предыдущие запросы"""
    neyra = Neyra()
    neyra.history.add("первый запрос")
    neyra.history.add("второй запрос")
    result = neyra.process_command("@Вспомни: 1@")
    assert "первый запрос" in result


def test_neyra_personality_traits():
    """Проверяю, что личность Нейры настроена правильно"""
    neyra = Neyra()
    
    # Проверяю основные черты
    assert 0 <= neyra.personality.curiosity_level <= 1
    assert 0 <= neyra.personality.creativity_boost <= 1
    assert 0 <= neyra.personality.empathy_factor <= 1
    
    # Любознательность должна быть высокой
    assert neyra.personality.curiosity_level >= 0.8


def test_neyra_responds_to_casual_text():
    """Нейра должна дружелюбно отвечать на обычный текст"""
    neyra = Neyra()
    
    casual_text = "Привет, как дела?"
    result = neyra.process_command(casual_text)
    
    assert len(result) > 0
    assert any(emoji in result for emoji in ['🤔', '💭', '✨'])


if __name__ == "__main__":
    pytest.main([__file__, "-v"])