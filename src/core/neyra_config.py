"""
Конфигурация личности Нейры.
Здесь живут все настройки того, какой должна быть Нейра.
"""
from dataclasses import dataclass
from typing import Dict
try:
    import torch  # type: ignore
except ModuleNotFoundError:  # pragma: no cover
    class _Cuda:
        @staticmethod
        def is_available() -> bool:
            return False

    class torch:  # type: ignore
        cuda = _Cuda()


@dataclass
class NeyraPersonality:
    """Черты характера Нейры"""
    curiosity_level: float = 0.9       # Любознательность (0-1)
    creativity_boost: float = 0.8      # Творческий потенциал
    empathy_factor: float = 0.85       # Эмпатия к персонажам
    attention_to_detail: float = 0.95  # Внимание к деталям
    humor_tendency: float = 0.3        # Склонность к юмору
    encouragement_level: float = 0.9   # Уровень поддержки


@dataclass
class TagSystemConfig:
    """Настройки системы тегов - сердца Нейры"""
    # Основные теги
    CORE_TAGS = {
        'neyra_command': r'@Нейра:\s*([^@]+)@',
        'memory_recall': r'@Вспомни:\s*([^@]+)@',
        'character_work': r'@Персонаж:\s*([^@]+)@',
        'style_guide': r'@Стиль:\s*([^@]+)@',
        'emotion_paint': r'@Эмоция:\s*([^@]+)@',
        'consistency_check': r'@Проверь:\s*([^@]+)@',
        'dialogue_create': r'@Диалог:\s*([^@]+)@',
        'scene_build': r'@Сцена:\s*([^@]+)@',
        'description_write': r'@Описание:\s*([^@]+)@'
    }

    # Дополнительные теги
    EXTENDED_TAGS = {
        'length_control': r'@Длина:\s*([^@]+)@',
        'tone_setting': r'@Тон:\s*([^@]+)@',
        'time_context': r'@Время:\s*([^@]+)@',
        'location_context': r'@Место:\s*([^@]+)@',
        'genre_guide': r'@Жанр:\s*([^@]+)@'
    }


@dataclass
class ModelConfig:
    """Настройки языковой модели для мозга Нейры"""
    model_name: str = "llama2-7b-chat"
    quantization: str = "4bit"
    device: str = "cuda" if torch.cuda.is_available() else "cpu"
    max_memory: Dict[str, str] = None


@dataclass
class GenerationConfig:
    """Как Нейра создает тексты"""
    temperature: float = 0.7
    top_p: float = 0.9
    max_length: int = 1000
    repetition_penalty: float = 1.1


@dataclass
class MemoryConfig:
    """Настройки памяти Нейры"""
    max_characters: int = 1000
    max_events: int = 10000
    pattern_threshold: int = 5
    emotional_memory_depth: int = 100


# Глобальные константы
NEYRA_VERSION = "0.1.0"
NEYRA_GREETING = "Привет! Я Нейра, ваш персональный помощник для творчества! ✨"
DEFAULT_ENCODING = "utf-8"
