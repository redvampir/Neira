"""Проверяю, понимает ли Нейра команды через теги"""

from src.tags.enhanced_parser import EnhancedTagParser


def test_Нейра_понимает_основные_теги() -> None:
    """Нейра должна понимать свой язык тегов"""
    parser = EnhancedTagParser()
    
    text = "@Нейра: Создай сцену@ @Эмоция: радость@"
    tags = parser.parse_user_input(text)
    
    assert len(tags) == 2
    assert tags[0].type == 'neyra_command'
    assert tags[1].type == 'emotion_paint'


def test_parser_understands_description_tag() -> None:
    parser = EnhancedTagParser()
    text = "@Описание: закат над морем@"
    tags = parser.parse_user_input(text)
    assert len(tags) == 1
    assert tags[0].type == 'description_write'


def test_parser_handles_style_example_block() -> None:
    parser = EnhancedTagParser()
    text = "[Пример стиля автора, Имя]\nТишина.\n[Пример окончен]"
    tags = parser.parse_user_input(text)
    assert len(tags) == 1
    assert tags[0].type == 'style_example'
    assert tags[0].content == 'Тишина.'
    assert tags[0].params.get('author') == 'Имя'


def test_parser_handles_world_rule_block() -> None:
    parser = EnhancedTagParser()
    text = "[Правило мира, Магия]\nНе использовать магию.\n[Правило окончено]"
    tags = parser.parse_user_input(text)
    assert len(tags) == 1
    assert tags[0].type == 'world_rule'
    assert tags[0].content == 'Не использовать магию.'
    assert tags[0].params.get('name') == 'Магия'


def test_parser_handles_character_reminder_block() -> None:
    parser = EnhancedTagParser()
    text = "[Напоминание персонажа, Лили]\nОна любит приключения.\n[Напоминание окончено]"
    tags = parser.parse_user_input(text)
    assert len(tags) == 1
    assert tags[0].type == 'character_reminder'
    assert tags[0].content == 'Она любит приключения.'
    assert tags[0].params.get('name') == 'Лили'


def test_parser_handles_generate_content_block() -> None:
    parser = EnhancedTagParser()
    text = "[Сгенерируй, стих]\nО лесах.\n[Генерация окончена]"
    tags = parser.parse_user_input(text)
    assert len(tags) == 1
    assert tags[0].type == 'generate_content'
    assert tags[0].content == 'О лесах.'
    assert tags[0].params.get('topic') == 'стих'
