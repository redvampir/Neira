"""Проверяю, понимает ли Нейра команды через теги"""

from src.tags.tag_parser import TagParser


def test_Нейра_понимает_основные_теги() -> None:
    """Нейра должна понимать свой язык тегов"""
    parser = TagParser()
    
    text = "@Нейра: Создай сцену@ @Эмоция: радость@"
    tags = parser.parse_user_input(text)
    
    assert len(tags) == 2
    assert tags[0].type == 'neyra_command'
    assert tags[1].type == 'emotion_paint'
