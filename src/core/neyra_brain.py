"""
Мозг Нейры - здесь я думаю и учусь.
"""
import json
import logging
from typing import List, Dict, Any
from pathlib import Path

from src.tags.tag_parser import TagParser, Tag
from src.tags.command_executor import CommandExecutor
from src.core.neyra_config import NEYRA_GREETING, NeyraPersonality
from src.utils.encoding_detector import detect_encoding
from src.llm.mistral_interface import MistralLLM
from src.interaction import RequestHistory
from src.memory import CharacterMemory


class Neyra:
    """Я Нейра, и здесь моя основная логика."""

    def __init__(self) -> None:
        """Просыпаюсь и готовлю свои модули."""
        self.logger = logging.getLogger(__name__)
        self.parser = TagParser()
        self.llm_max_tokens = 512
        self.llm = self._load_llm()
        self.executor = CommandExecutor(self)
        self.personality = NeyraPersonality()
        self.known_books: List[str] = []
        self.characters_memory = CharacterMemory()
        self.emotional_state = "любопытная"
        self.history = RequestHistory()

        self.logger.info("Нейра проснулась! ✨")

    def _load_llm(self) -> MistralLLM | None:
        """Загружаю локальную LLM при наличии конфига."""
        config_path = Path("config/llm_config.json")
        if not config_path.exists():
            return None
        try:
            cfg = json.loads(config_path.read_text(encoding="utf-8"))
            model_path = cfg.get("model_path")
            self.llm_max_tokens = int(cfg.get("max_tokens", 512))
            return MistralLLM(model_path)
        except Exception as e:  # pragma: no cover
            self.logger.error(f"Ошибка загрузки LLM: {e}")
            return None

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

    def load_book(self, path: str) -> None:
        """Загружаю книгу в свою память с радостью открытия."""
        try:
            file_path = Path(path)
            if not file_path.exists():
                self.logger.warning(f"Книга не найдена: {path}")
                return

            # Определяю кодировку
            encoding = detect_encoding(path)
            content = file_path.read_text(encoding=encoding)

            self.known_books.append(path)
            self._extract_characters(content)

            print(f"📚 Изучила книгу: {file_path.name}")
            print(f"   Страниц текста: {len(content) // 2000}")
            if self.characters_memory:
                print(f"   Встретила персонажей: {len(self.characters_memory)}")

        except Exception as e:
            self.logger.error(f"Ошибка при загрузке {path}: {e}")

    def _extract_characters(self, content: str) -> None:
        """Ищу персонажей в тексте - моя любимая задача!"""
        # Простейшая эвристика: имена с заглавной буквы
        import re

        # Ищем слова с заглавной буквы (потенциальные имена)
        potential_names = re.findall(r'\b[А-ЯЁA-Z][а-яёa-z]+(?:\s+[А-ЯЁA-Z][а-яёa-z]+)?\b', content)

        # Фильтруем очевидно не-имена
        stop_words = {'Но', 'И', 'А', 'В', 'На', 'Он', 'Она', 'Они', 'Это', 'Что', 'Как', 'Где'}

        for name in potential_names:
            if name not in stop_words and len(name) >= 3:
                if name not in self.characters_memory:
                    self.characters_memory.add(
                        name,
                        {
                            "first_mention": True,
                            "personality_traits": [],
                            "emotional_moments": [],
                        },
                    )
        self.characters_memory.save()

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

    def process_command(self, text: str) -> str:
        """Обрабатываю команды с пониманием и творчеством."""
        tags = self.parser.parse_user_input(text)

        if not tags:
            return self._casual_response(text)

        # Создаю контекст для исполнителя
        context = {
            "emotion": self.emotional_state,
            "characters": list(self.characters_memory.keys()),
            "known_books": self.known_books,
        }

        response_parts = []

        for tag in tags:
            result = self.executor.execute_command(tag, context)
            response_parts.append(result)

        return "\n\n".join(response_parts) if response_parts else "💭 Хм, интересная команда! Обдумываю..."

    def _execute_neyra_command(self, command: str) -> str:
        """Выполняю прямые команды с энтузиазмом!"""
        command_lower = command.lower()

        if 'сцена' in command_lower or 'создай' in command_lower:
            return self._create_scene(command)
        elif 'диалог' in command_lower:
            return self._create_dialogue(command)
        elif 'персонаж' in command_lower:
            return self._analyze_character(command)
        else:
            return f"✨ Команда понята: '{command}'. Размышляю над выполнением..."

    def _create_scene(self, description: str) -> str:
        """Создаю сцену с творческим подходом."""
        templates = [
            "Туман стелился по земле, скрывая тайны наступающего утра...",
            "В комнате царила та особенная тишина, которая предшествует важным разговорам...",
            "Солнечные лучи пробивались сквозь листву, создавая причудливую игру света и тени..."
        ]

        import random
        base_scene = random.choice(templates)

        return f"🎨 Создаю сцену: {description}\n\n{base_scene}\n\n(Это базовый пример - скоро я научусь создавать уникальные сцены!)"

    def _create_dialogue(self, description: str) -> str:
        """Создаю диалог как временную заглушку."""
        return f"💬 Создаю диалог: {description}"

    def _analyze_character(self, description: str) -> str:
        """Анализирую персонажа как временную заглушку."""
        return f"🔎 Анализ персонажа: {description}"

    def _work_with_character(self, character_info: str) -> str:
        """Работаю с персонажем, помня его как живого."""
        if ':' in character_info:
            name, action = character_info.split(':', 1)
            name = name.strip()
            action = action.strip()
        else:
            name = character_info.strip()
            action = "общая информация"

        if name in self.characters_memory:
            return f"👤 {name} - помню этого персонажа! {action}"
        else:
            # Добавляю нового персонажа
            self.characters_memory.add(
                name,
                {
                    "personality_traits": [],
                    "emotional_moments": [],
                },
            )
            self.characters_memory.save()
            return f"👤 Знакомлюсь с {name}! Запоминаю: {action}"

    def _add_emotion(self, emotion: str) -> str:
        """Добавляю эмоциональную окраску."""
        self.emotional_state = emotion
        return f"💭 Настраиваюсь на эмоцию: {emotion}"

    def _apply_style(self, style: str) -> str:
        """Адаптируюсь под стиль."""
        return f"🎭 Подстраиваюсь под стиль: {style}"

    def _casual_response(self, text: str) -> str:
        """Отвечаю на обычный текст дружелюбно."""
        responses = [
            "🤔 Интересная мысль! Хотите использовать теги для более точной работы?",
            "💭 Понимаю! Попробуйте команду @Нейра: что вы хотите@",
            "✨ Я здесь! Используйте теги, чтобы я лучше поняла, чем помочь."
        ]

        import random
        return random.choice(responses)
