"""Мозг Нейры - здесь я думаю и учусь."""
import logging
from typing import List, Dict, Any
from pathlib import Path

from src.tags.tag_parser import TagParser
from src.tags.command_executor import CommandExecutor
from src.core.neyra_config import NEYRA_GREETING, NeyraPersonality
from src.utils.encoding_detector import detect_encoding


class Neyra:
    """Я Нейра, и здесь моя основная логика."""

    def __init__(self) -> None:
        """Просыпаюсь и готовлю свои модули."""
        self.logger = logging.getLogger(__name__)
        self.parser = TagParser()
        self.personality = NeyraPersonality()
        self.known_books: List[str] = []
        self.characters_memory: Dict[str, Dict[str, Any]] = {}
        self.emotional_state = "любопытная"

        # Улучшенный исполнитель команд позволяет гибко обрабатывать теги
        self.executor = CommandExecutor(self)

        self.logger.info("Нейра проснулась! ✨")

    # ------------------------------------------------------------------
    # Пользовательское взаимодействие
    def introduce_yourself(self) -> None:
        """Представляюсь пользователю с энтузиазмом!"""
        print(f"\n{NEYRA_GREETING}")
        print("🎭 Я умею:")
        print("   • Анализировать ваши тексты с эмоциональным пониманием")
        print("   • Создавать живые диалоги и описания")
        print("   • Помнить каждого персонажа как живого человека")
        print("   • Понимать команды через систему тегов")
        print("\n🏷️ Используйте теги для общения со мной:")
        print("   @Нейра: что сделать@")
        print("   @Персонаж: имя - описание@")
        print("   @Эмоция: чувство@")
        print("   @Стиль: как писать@\n")

    # ------------------------------------------------------------------
    # Работа с книгами и памятью
    def load_book(self, path: str) -> None:
        """Загружаю книгу в свою память с радостью открытия."""
        try:
            file_path = Path(path)
            if not file_path.exists():
                self.logger.warning(f"Книга не найдена: {path}")
                return

            encoding = detect_encoding(path)
            content = file_path.read_text(encoding=encoding)

            self.known_books.append(path)
            self._extract_characters(content)

            print(f"📚 Изучила книгу: {file_path.name}")
            print(f"   Страниц текста: {len(content) // 2000}")
            if self.characters_memory:
                print(f"   Встретила персонажей: {len(self.characters_memory)}")

        except Exception as e:  # pragma: no cover
            self.logger.error(f"Ошибка при загрузке {path}: {e}")

    def _extract_characters(self, content: str) -> None:
        """Ищу персонажей в тексте - моя любимая задача!"""
        import re

        potential_names = re.findall(r'\b[А-ЯЁA-Z][а-яёa-z]+(?:\s+[А-ЯЁA-Z][а-яёa-z]+)?\b', content)
        stop_words = {'Но', 'И', 'А', 'В', 'На', 'Он', 'Она', 'Они', 'Это', 'Что', 'Как', 'Где'}

        for name in potential_names:
            if name not in stop_words and len(name) >= 3:
                if name not in self.characters_memory:
                    self.characters_memory[name] = {
                        'first_mention': True,
                        'personality_traits': [],
                        'emotional_moments': []
                    }

    def analyze_content(self) -> None:
        """Анализирую загруженные книги с энтузиазмом исследователя."""
        if not self.known_books:
            print("😔 Пока нет книг для анализа...")
            return

        print("🔍 Провожу глубокий анализ...")
        print(f"   Изучено книг: {len(self.known_books)}")
        print(f"   Персонажей в памяти: {len(self.characters_memory)}")
        if self.characters_memory:
            print("   Главные герои:", list(self.characters_memory.keys())[:5])

    # ------------------------------------------------------------------
    # Обработка команд
    def process_command(self, text: str) -> str:
        """Обрабатываю команды с пониманием и творчеством."""
        tags = self.parser.parse_user_input(text)

        if not tags:
            return self._casual_response(text)

        responses = [self.executor.execute_command(tag, {}) for tag in tags]
        return "\n".join(responses) if responses else "💭 Хм, интересная команда! Обдумываю..."

    def _casual_response(self, text: str) -> str:
        """Отвечаю на обычный текст дружелюбно."""
        responses = [
            "🤔 Интересная мысль! Хотите использовать теги для более точной работы?",
            "💭 Понимаю! Попробуйте команду @Нейра: что вы хотите@",
            "✨ Я здесь! Используйте теги, чтобы я лучше поняла, чем помочь.",
        ]

        import random
        return random.choice(responses)
