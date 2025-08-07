"""Менеджер тегов и обработчиков.

Этот модуль предоставляет простой реестр, который связывает имя тега
с функцией-обработчиком. Он позволяет централизованно управлять тегами и
служит точкой расширения для сторонних плагинов.

Плагин может зарегистрировать свой тег, импортировав функцию
``register_tag`` и передав ей имя тега и функцию-обработчик:

    from src.tags.manager import register_tag

    def my_handler(content: str, context: dict) -> str:
        return "handled:" + content

    register_tag("my_tag", my_handler)

Обработчик должен принимать два аргумента: содержимое тега и словарь
контекста, а возвращать строку-результат.
"""

import re
from typing import Callable, Dict, Any, Optional

from src.iteration.strategy_manager import AdaptiveIterationManager

# Словари реестра
_patterns: Dict[str, str] = {}
_handlers: Dict[str, Callable[[str, Dict[str, Any]], str]] = {}


def register_pattern(tag_name: str, pattern: str) -> None:
    """Регистрирует регулярное выражение для разбора тега."""
    _patterns[tag_name] = pattern


def register_handler(tag_name: str, handler: Callable[[str, Dict[str, Any]], str]) -> None:
    """Регистрирует обработчик для заданного типа тега."""
    _handlers[tag_name] = handler


def register_tag(
    tag_name: str,
    handler: Callable[[str, Dict[str, Any]], str],
    *,
    pattern: Optional[str] = None,
) -> None:
    """Регистрация тега целиком.

    Используется как в основной системе, так и внешними плагинами.
    """
    if pattern is not None:
        register_pattern(tag_name, pattern)
    register_handler(tag_name, handler)


def get_handler(tag_name: str) -> Optional[Callable[[str, Dict[str, Any]], str]]:
    """Возвращает обработчик для указанного тега, если он есть."""
    return _handlers.get(tag_name)


def get_patterns() -> Dict[str, str]:
    """Возвращает копию реестра шаблонов тегов."""
    return dict(_patterns)


def available_tags() -> Dict[str, Callable[[str, Dict[str, Any]], str]]:
    """Возвращает копию реестра обработчиков."""
    return dict(_handlers)


def iteration_strategy_handler(mode: str, context: Dict[str, Any]) -> str:
    """Handle iteration strategy selection tag.

    Parameters
    ----------
    mode:
        Strategy preset name provided inside the tag.
    context:
        Execution context which must contain the original user query under
        ``"query"`` and a reference to the :class:`Neyra` instance under
        ``"neyra"``.
    """

    neyra = context.get("neyra")
    query = context.get("query", "")
    strategy = AdaptiveIterationManager.determine_strategy(mode.strip().lower())
    clean_query = re.sub(r"@Итерация:\s*[^@]+@", "", query, flags=re.IGNORECASE).strip()
    if neyra and hasattr(neyra, "iterative_response"):
        result, _ = neyra.iterative_response(clean_query, strategy)
        return result
    return "🤔 Команда итерации недоступна."


register_tag(
    "iteration_strategy",
    iteration_strategy_handler,
    pattern=r"@Итерация:\s*([^@]+)@",
)


def min_iterations_handler(value: str, context: Dict[str, Any]) -> str:
    """Handle tag to set minimum iteration count dynamically."""

    neyra = context.get("neyra")
    try:
        count = int(value.strip())
    except ValueError:
        return "⚠️ Некорректное значение минимума итераций."

    if neyra:
        neyra.config.min_iterations = count
        if hasattr(neyra, "iteration_controller"):
            neyra.iteration_controller.min_iterations = count
    return f"🔁 Минимальное количество итераций установлено: {count}"


register_tag(
    "min_iterations",
    min_iterations_handler,
    pattern=r"@Минимум:\s*(\d+)@",
)
