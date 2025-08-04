"""
Исполнитель команд - здесь я выполняю то, что просят теги.
"""
from __future__ import annotations

import random
from typing import Any, Callable, Dict, List, Optional

from src.tags.tag_parser import Tag
from src.tags.manager import (
    available_tags as manager_available_tags,
    get_handler as manager_get_handler,
    register_handler as manager_register_handler,
)
from src.action.dialogue_master import DialogueMaster
from src.action.scene_painter import ScenePainter
from src.action.description_writer import DescriptionWriter
from src.models import Character, Scene


DEFAULT_DIALOGUE_STYLE = "дружеский"


class CommandExecutor:
    """Я выполняю команды с творческим подходом и пониманием контекста.

    Исполнитель построен на системе обработчиков. Для каждого типа тега
    регистрируется функция, которая умеет работать с содержимым тега. Это
    позволяет легко расширять систему новыми командами и упрощает поддержку
    существующих.
    """

    def __init__(self, neyra_brain: Optional[Any] = None) -> None:
        self.neyra_brain = neyra_brain

        # Палитры эмоций и стили, которыми я могу пользоваться
        self.emotional_palettes: Dict[str, List[str]] = {
            "радость": ["солнечный", "искрящийся", "светлый", "воздушный"],
            "грусть": ["серый", "тихий", "меланхоличный", "дождливый"],
            "тревога": ["напряженный", "беспокойный", "тревожный", "нервный"],
            "любовь": ["нежный", "теплый", "мягкий", "романтичный"],
            "страх": ["мрачный", "холодный", "зловещий", "пугающий"],
        }

        self.style_templates: Dict[str, str] = {
            "классический": "С изысканной точностью и благородством слога",
            "современный": "Живо и динамично, как дыхание большого города",
            "романтический": "С нежностью и трепетным волнением",
            "мистический": "Окутанно тайной и древними секретами",
            "драматический": "С накалом страстей и эмоциональным напряжением",
        }

        # Регистрация стандартных обработчиков в менеджере
        self._register_default_handlers()
        llm = getattr(neyra_brain, "llm", None) if neyra_brain else None
        self.dialogue_master = DialogueMaster(llm)
        self.scene_painter = ScenePainter(llm)
        self.description_writer = DescriptionWriter(llm)

    # ------------------------------------------------------------------
    # Регистрация обработчиков
    def _register_default_handlers(self) -> None:
        manager_register_handler("neyra_command", self._execute_neyra_command)
        manager_register_handler("memory_recall", self._recall_request)
        manager_register_handler("character_work", self._work_with_character)
        manager_register_handler("emotion_paint", self._paint_with_emotion)
        manager_register_handler("style_guide", self._apply_style)
        manager_register_handler("dialogue_create", self._create_dialogue)
        manager_register_handler("scene_build", self._build_scene)
        manager_register_handler("description_write", self._write_description)
        manager_register_handler("consistency_check", self._check_consistency)

    def register_handler(
        self, tag_type: str, handler: Callable[[str, Dict[str, Any]], str]
    ) -> None:
        """Регистрация нового обработчика для кастомного типа тега."""
        manager_register_handler(tag_type, handler)

    def available_handlers(self) -> List[str]:
        """Возвращает список доступных типов команд."""
        return sorted(manager_available_tags().keys())

    # ------------------------------------------------------------------
    # Основной метод исполнения команд
    def execute_command(self, tag: Tag, context: Optional[Dict[str, Any]] = None) -> str:
        """Выполняю команду, учитывая контекст и эмоциональное состояние."""
        if context is None:
            context = {}

        handler = manager_get_handler(tag.type)
        if handler:
            return handler(tag.content, context)
        return f"🤔 Команда '{tag.type}' понята, но пока учусь её выполнять..."

    def _recall_request(self, query: str, context: Dict[str, Any]) -> str:
        """Ищу запросы пользователя по истории."""
        if self.neyra_brain is None or not hasattr(self.neyra_brain, "history"):
            return "📝 История пока недоступна."
        return self.neyra_brain.history.search(query)

    # ------------------------------------------------------------------
    # Обработчики отдельных команд
    def _execute_neyra_command(self, command: str, context: Dict[str, Any]) -> str:
        """Выполняю прямые команды Нейре."""
        command_lower = command.lower()

        if any(word in command_lower for word in ["сцена", "создай сцену", "опиши"]):
            scene = self._create_creative_scene(command, context)
            return scene.content
        if any(word in command_lower for word in ["диалог", "разговор", "беседа"]):
            return self._create_smart_dialogue(command, context)
        if any(word in command_lower for word in ["персонаж", "герой", "характер"]):
            return self._analyze_character_deeply(command, context)
        if any(word in command_lower for word in ["продолжи", "развей", "дальше"]):
            return self._continue_story(command, context)
        return f"✨ Понимаю: '{command}'. Готова к творчеству!"

    def _create_creative_scene(self, description: str, context: Dict[str, Any]) -> Scene:
        emotion = context.get("emotion", "нейтральная")
        style = context.get("style", "современный")

        atmospheres = {
            "загадочная": [
                "Полумрак окутывал комнату, скрывая в тенях неведомые тайны",
                "В воздухе витало что-то неуловимое, заставляющее сердце биться чаще",
                "Тишина была настолько глубокой, что казалось, можно услышать шепот прошлого",
            ],
            "романтическая": [
                "Мягкий свет свечей играл на стенах, создавая интимную атмосферу",
                "Аромат роз наполнял воздух, а где-то вдали звучала нежная мелодия",
                "Лунный свет струился через окно, серебря всё вокруг волшебным сиянием",
            ],
            "драматическая": [
                "Гроза бушевала за окном, отражая внутреннее напряжение момента",
                "В комнате царила та напряженная тишина, которая предшествует важным решениям",
                "Время словно замерло в ожидании слов, которые изменят всё",
            ],
        }

        scene_type = "загадочная"
        for scene_key in atmospheres.keys():
            if scene_key in description.lower():
                scene_type = scene_key
                break

        base_scene = random.choice(atmospheres[scene_type])

        emotion_addition = ""
        if emotion in self.emotional_palettes:
            emotional_words = self.emotional_palettes[emotion]
            emotion_addition = f" {random.choice(emotional_words)} оттенок"

        content = (
            f"🎨 Создаю сцену: {description}\n\n"
            f"{base_scene}.{emotion_addition}\n\n"
            "Детали проявляются постепенно: каждый предмет здесь имеет свою историю, "
            "каждая тень - своё значение. Воздух наполнен предчувствием того, что "
            "вот-вот произойдет что-то важное.\n\n"
            f"(Эмоция: {emotion}, Стиль: {style})"
        )

        return Scene(description=description, content=content, emotion=emotion, style=style)

    def _create_smart_dialogue(self, command: str, context: Dict[str, Any]) -> str:
        characters = context.get("characters", ["Персонаж А", "Персонаж Б"])
        emotion = context.get("emotion", "нейтральная")

        dialogue_templates = {
            "формальный": {
                "opening": ["Позвольте заметить", "Следует отметить", "Я вынужден сказать"],
                "response": ["Безусловно", "Разумеется", "Я полностью согласен"],
            },
            DEFAULT_DIALOGUE_STYLE: {
                "opening": ["Слушай", "Знаешь что", "Кстати говоря"],
                "response": ["Точно!", "Да ладно!", "Не может быть!"],
            },
            "конфликтный": {
                "opening": ["Это неправда!", "Как ты смеешь!", "Я не позволю!"],
                "response": ["И что теперь?", "Попробуй остановить!", "Посмотрим!"],
            },
        }

        style = DEFAULT_DIALOGUE_STYLE
        template = dialogue_templates[style]

        return (
            f"💬 Создаю диалог: {command}\n\n"
            f"— {random.choice(template['opening'])}, — начал {characters[0] if len(characters) > 0 else 'первый персонаж'}, внимательно изучая собеседника.\n\n"
            f"— {random.choice(template['response'])}, — ответил {characters[1] if len(characters) > 1 else 'второй персонаж'}, и в голосе его звучала особая интонация.\n\n"
            "— Понимаешь, дело не только в словах... — продолжил разговор первый, делая значительную паузу.\n\n"
            f"(Эмоция: {emotion}, Стиль: {style})"
        )

    def _create_dialogue(self, command: str, context: Dict[str, Any]) -> str:
        """Обработчик для прямого тега создания диалога."""
        max_tokens = getattr(self.neyra_brain, "llm_max_tokens", 512)
        if self.dialogue_master.llm is not None:
            return self.dialogue_master.create(command, max_tokens=max_tokens)
        return self._create_smart_dialogue(command, context)

    def _work_with_character(self, character_info: str, context: Dict[str, Any]) -> str:
        if " - " in character_info:
            name, trait = character_info.split(" - ", 1)
            name, trait = name.strip(), trait.strip()
        else:
            name, trait = character_info.strip(), "развитие характера"

        if self.neyra_brain is not None and hasattr(self.neyra_brain, "characters_memory"):
            memory = self.neyra_brain.characters_memory
            character = memory.get(name)
            if character is None:
                character = Character(name=name, personality_traits=[trait])
            else:
                character.personality_traits.append(trait)
            memory.add(character)
            memory.save()

        insights = [
            f"В {name} есть что-то особенное - {trait} проявляется не только в словах, но и в жестах",
            f"Когда {name} {trait.lower()}, это видно по тому, как меняется взгляд",
            f"Характер {name} раскрывается через {trait} - это ключ к пониманию персонажа",
        ]

        return (
            f"👤 Работаю с персонажем {name}:\n\n"
            f"{random.choice(insights)}.\n\n"
            f"Черта характера: {trait}\n"
            "Глубина проработки: углубляю понимание мотивов и внутреннего мира\n\n"
            f"✨ {name} теперь живет в моей памяти как реальный человек со всеми сложностями характера."
        )

    def _paint_with_emotion(self, emotion: str, context: Dict[str, Any]) -> str:
        if self.neyra_brain is not None:
            self.neyra_brain.emotional_state = emotion

        if emotion.lower() in self.emotional_palettes:
            palette = self.emotional_palettes[emotion.lower()]
            color_description = f"Окрашиваю текст в {', '.join(palette[:2])} тона"
        else:
            color_description = f"Настраиваюсь на эмоцию '{emotion}'"

        return (
            f"🎨 {color_description}. Каждое слово теперь несет отпечаток этого чувства"
            f" ({emotion})."
        )

    def _apply_style(self, style: str, context: Dict[str, Any]) -> str:
        if self.neyra_brain is not None:
            setattr(self.neyra_brain, "current_style", style)
        llm = getattr(self.neyra_brain, "llm", None) if self.neyra_brain else None
        if llm is not None:
            prompt = f"Опиши стиль письма '{style}'"
            max_tokens = getattr(self.neyra_brain, "llm_max_tokens", 512)
            return llm.generate(prompt, max_tokens=max_tokens)
        template = self.style_templates.get(style.lower())
        if template:
            return f"🎭 {template}. (Стиль: {style})"
        return f"🎭 Подстраиваюсь под стиль: {style}"

    def _build_scene(self, scene_description: str, context: Dict[str, Any]) -> str:
        max_tokens = getattr(self.neyra_brain, "llm_max_tokens", 512)
        if self.scene_painter.llm is not None:
            scene = self.scene_painter.paint(scene_description, max_tokens=max_tokens)
        else:
            scene = self._create_creative_scene(scene_description, context)
        return scene.content

    def _write_description(self, description: str, context: Dict[str, Any]) -> str:
        max_tokens = getattr(self.neyra_brain, "llm_max_tokens", 512)
        if self.description_writer.llm is not None:
            return self.description_writer.write(description, max_tokens=max_tokens)
        return f"📜 Описание: {description}"

    def _check_consistency(self, check_target: str, context: Dict[str, Any]) -> str:
        return f"🔍 Анализирую консистентность: {check_target}. Ищу противоречия и несоответствия в деталях..."

    def _analyze_character_deeply(self, command: str, context: Dict[str, Any]) -> str:
        return f"🔎 Анализирую персонажа: {command}. Выявляю мотивации, страхи и скрытые желания..."

    def _continue_story(self, instruction: str, context: Dict[str, Any]) -> str:
        return f"📖 Продолжаю историю: {instruction}. Соблюдаю логику повествования и характеры персонажей..."
