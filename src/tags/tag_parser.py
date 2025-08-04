"""
Система тегов - это язык общения с Нейрой.
Здесь я учусь понимать команды пользователя.
"""
import re
from typing import List, Dict, Optional
from dataclasses import dataclass

@dataclass
class Tag:
    """Один тег - одна команда для Нейры"""
    type: str           # Тип команды
    content: str        # Содержание команды  
    position: tuple     # Позиция в тексте
    priority: int = 1   # Приоритет выполнения

class TagParser:
    """Я умею понимать команды через теги!"""
    
    def __init__(self) -> None:
        """Инициализирую свой словарь понимания тегов"""
        from src.core.neyra_config import TagSystemConfig
        self.tag_patterns: Dict[str, str] = {
            **TagSystemConfig.CORE_TAGS,
            **TagSystemConfig.EXTENDED_TAGS
        }
    
    def parse_user_input(self, text: str) -> List[Tag]:
        """
        Разбираю пользовательский текст на понятные мне команды.
        
        Например: '@Нейра: Создай сцену@ @Эмоция: грусть@'
        Я пойму это как две команды!
        """
        tags: List[Tag] = []
        
        for tag_type, pattern in self.tag_patterns.items():
            matches = re.finditer(pattern, text, re.IGNORECASE)
            for match in matches:
                tag = Tag(
                    type=tag_type,
                    content=match.group(1).strip(),
                    position=match.span()
                )
                tags.append(tag)
        
        # Сортирую по позиции в тексте
        return sorted(tags, key=lambda x: x.position[0])
    
    def suggest_tags(self, context: str) -> List[str]:
        """Предлагаю теги, которые могут быть полезны"""
        suggestions: List[str] = []
        
        # Если упоминается персонаж, предлагаю @Персонаж:@
        if any(word.istitle() for word in context.split()):
            suggestions.append("@Персонаж: имя - действие@")
            
        # Если текст эмоциональный, предлагаю @Эмоция:@
        emotion_words = ['грустно', 'радостно', 'страшно', 'весело']
        if any(word in context.lower() for word in emotion_words):
            suggestions.append("@Эмоция: чувство@")
            
        return suggestions
