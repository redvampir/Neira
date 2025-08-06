from __future__ import annotations

"""Prompt templates with user style adaptation."""

from src.memory.style_memory import StyleMemory, StylePattern


DEFAULT_CHAT_TEMPLATE = "{prompt}"


def apply_user_style(
    prompt: str,
    *,
    user_id: str | None = None,
    style_memory: StyleMemory | None = None,
) -> str:
    """Return ``prompt`` adapted according to ``user_id`` style.

    Parameters
    ----------
    prompt:
        Original prompt text.
    user_id:
        Identifier of the user whose style should be applied.
    style_memory:
        Optional :class:`StyleMemory` used to fetch style information.
    """

    if user_id is None:
        return prompt
    memory = style_memory or StyleMemory()
    style = memory.get_style(user_id, "preferred")
    if not isinstance(style, StylePattern):
        return prompt
    style_prefix = ""
    if style.description:
        style_prefix += f"Тон: {style.description}\n"
    if style.examples:
        style_prefix += "Примеры:\n" + "\n".join(style.examples) + "\n"
    return style_prefix + prompt


def chat_prompt(
    prompt: str,
    *,
    user_id: str | None = None,
    style_memory: StyleMemory | None = None,
) -> str:
    """Format a basic chat prompt applying user style if available."""

    base = DEFAULT_CHAT_TEMPLATE.format(prompt=prompt)
    return apply_user_style(base, user_id=user_id, style_memory=style_memory)


__all__ = ["chat_prompt", "apply_user_style", "DEFAULT_CHAT_TEMPLATE"]
