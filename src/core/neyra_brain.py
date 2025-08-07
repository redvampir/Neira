"""
Мозг Нейры - здесь я думаю и учусь.
"""
import json
import logging
from typing import List, Dict, Any
from pathlib import Path

from src.tags.enhanced_parser import EnhancedTagParser as TagParser, Tag
from src.tags.command_executor import CommandExecutor
from src.core.neyra_config import NEYRA_GREETING, NeyraPersonality
from src.utils.encoding_detector import detect_encoding
from src.llm import BaseLLM, LLMFactory
from src.interaction import RequestHistory
from src.memory import CharacterMemory, WorldMemory, StyleMemory
from src.analysis import VerificationSystem, VerificationResult, UncertaintyManager
from src.iteration import (
    DraftGenerator,
    GapAnalyzer,
    DeepSearcher,
    ResponseEnhancer,
    IntegrationType,
    IterationController,
)
from src.models import Character
from src.core.cache_manager import CacheManager


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
        self.world_memory = WorldMemory()
        self.style_memory = StyleMemory()
        self.draft_generator = DraftGenerator(
            self.characters_memory, self.world_memory, self.style_memory
        )
        self.last_draft: str = ""
        self.verification_system = VerificationSystem()
        self.uncertainty_manager = UncertaintyManager()
        self.gap_analyzer = GapAnalyzer(self.verification_system, self.uncertainty_manager)
        self.deep_searcher = DeepSearcher(
            self.characters_memory, self.world_memory, self.style_memory
        )
        self.response_enhancer = ResponseEnhancer()
        self.iteration_controller = IterationController()
        self.current_user_id = "default"
        self.current_style = ""
        self.emotional_state = "любопытная"
        self.history = RequestHistory(load_existing=False)
        self.cache = CacheManager()

        self.logger.info("Нейра проснулась! ✨")

    def _load_llm(self) -> BaseLLM | None:
        """Загружаю локальную LLM при наличии конфига."""
        config_path = Path("config/llm_config.json")
        if not config_path.exists():
            return None
        try:
            raw_config = config_path.read_text(encoding="utf-8")
            cfg = json.loads(raw_config)
        except (OSError, UnicodeDecodeError) as e:  # pragma: no cover
            self.logger.error(
                "Ошибка чтения файла конфигурации LLM %s: %s", config_path, e
            )
            return None
        except json.JSONDecodeError as e:  # pragma: no cover
            self.logger.error(
                "Некорректный JSON в конфиге LLM %s: %s", config_path, e
            )
            return None

        model_type = cfg.get("model_type", "mistral")
        model_path = cfg.get("model_path")
        self.llm_max_tokens = int(cfg.get("max_tokens", 512))
        try:
            return LLMFactory.create(model_type, model_path=model_path)
        except (ValueError, RuntimeError, OSError) as e:  # pragma: no cover
            self.logger.error(
                "Ошибка инициализации LLM %s из %s: %s", model_type, model_path, e
            )
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
        file_path = Path(path)
        if not file_path.exists():
            self.logger.warning(f"Книга не найдена: {path}")
            return

        cache_key = f"load_book:{file_path}"
        try:
            mtime = file_path.stat().st_mtime
            cached = self.cache.get(cache_key)
            if cached and cached.get("mtime") == mtime:
                self.known_books.append(path)
                print(f"📚 Изучила книгу из кэша: {file_path.name}")
                return

            encoding = detect_encoding(path)
            content = file_path.read_text(encoding=encoding)
        except OSError as e:
            self.logger.error("Ошибка чтения файла %s: %s", path, e)
            return
        except UnicodeDecodeError as e:
            self.logger.error("Ошибка декодирования файла %s: %s", path, e)
            return

        self.known_books.append(path)
        self._extract_characters(content)

        print(f"📚 Изучила книгу: {file_path.name}")
        print(f"   Страниц текста: {len(content) // 2000}")
        if self.characters_memory:
            print(f"   Встретила персонажей: {len(self.characters_memory)}")

        self.cache.set(cache_key, {"mtime": mtime})

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
                        Character(
                            name=name,
                            personality_traits=[],
                            emotional_moments=[],
                            relationships={},
                            growth_arc=[],
                            first_mention=True,
                        )
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

    def verify_claim(self, claim: str) -> VerificationResult:
        """Проверяю утверждение с помощью системы верификации."""
        result = self.verification_system.verify_claim(claim)
        result = self.uncertainty_manager.handle(result)
        if result.confidence < self.uncertainty_manager.threshold:
            result.clarifying_questions = self.verification_system.generate_clarifying_questions(claim)
        return result

    def process_command(self, text: str) -> str:
        """Обрабатываю команды с пониманием и творчеством."""
        self.last_draft = self.draft_generator.generate_draft(
            text, self.verification_system.memory
        )
        tags = self.parser.parse_user_input(text)

        if not tags:
            return self._casual_response(text)

        # Создаю контекст для исполнителя
        context = {
            "emotion": self.emotional_state,
            "characters": list(self.characters_memory.keys()),
            "known_books": self.known_books,
            "worlds": self.world_memory.get(),
            "style_examples": [],
        }

        response_parts = []

        for tag in tags:
            result = self.executor.execute_command(tag, context)
            response_parts.append(result)

        if context.get("style_examples"):
            user_id = getattr(self, "current_user_id", "default")
            for example in context["style_examples"]:
                self.style_memory.add(
                    user_id, self.current_style or "общий", example=example
                )
            self.style_memory.save()

        return "\n\n".join(response_parts) if response_parts else "💭 Хм, интересная команда! Обдумываю..."

    def iterative_response(self, query: str) -> str:
        """Return a refined response using iterative improvement pipeline."""
        response = self.process_command(query)
        draft = self.last_draft or response
        while True:
            gaps = self.gap_analyzer.analyze(draft)
            if not gaps:
                break
            search_results: List[Dict[str, Any]] = []
            for gap in gaps:
                try:
                    search_results.extend(
                        self.deep_searcher.search(
                            gap.claim, user_id=getattr(self, "current_user_id", "default")
                        )
                    )
                except Exception:
                    continue
            response = self.response_enhancer.enhance(
                response, search_results, IntegrationType.IMPORTANT_ADDITION
            )
            draft = response
            if not self.iteration_controller.should_iterate(response):
                break
        return response

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
        cache_key = f"scene:{description}"
        cached = self.cache.get(cache_key)
        if cached:
            return cached
        templates = [
            "Туман стелился по земле, скрывая тайны наступающего утра...",
            "В комнате царила та особенная тишина, которая предшествует важным разговорам...",
            "Солнечные лучи пробивались сквозь листву, создавая причудливую игру света и тени...",
        ]
        import random
        base_scene = random.choice(templates)
        scene = (
            f"🎨 Создаю сцену: {description}\n\n{base_scene}\n\n(Это базовый пример - скоро я научусь создавать уникальные сцены!)"
        )
        self.cache.set(cache_key, scene)
        return scene

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
                Character(name=name, personality_traits=[], emotional_moments=[])
            )
            self.characters_memory.save()
            return f"👤 Знакомлюсь с {name}! Запоминаю: {action}"

    def remember_world(self, name: str, info: Dict[str, Any]) -> None:
        """Сохраняю информацию о мире."""
        self.world_memory.add(name, info)
        self.world_memory.save()

    def get_world(self, name: str | None = None) -> Any:
        """Возвращаю сведения о мире."""
        return self.world_memory.get(name)

    def remember_style(
        self,
        style: str,
        example: str | None = None,
        description: str | None = None,
    ) -> None:
        """Запоминаю стиль письма и его примеры."""
        user_id = getattr(self, "current_user_id", "default")
        self.style_memory.add(user_id, style, example=example, description=description)
        self.style_memory.save()

    def get_style(self, style: str | None = None) -> Any:
        """Возвращаю сведения о стилях."""
        user_id = getattr(self, "current_user_id", "default")
        return self.style_memory.get_style(user_id, style)

    def _add_emotion(self, emotion: str) -> str:
        """Добавляю эмоциональную окраску."""
        self.emotional_state = emotion
        return f"💭 Настраиваюсь на эмоцию: {emotion}"

    def _apply_style(self, style: str) -> str:
        """Адаптируюсь под стиль и запоминаю его."""
        self.current_style = style
        self.remember_style(style)
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
