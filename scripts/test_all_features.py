"""Run a small showcase of Neyra's capabilities.

This script instantiates :class:`Neyra`, checks that the local language model
is available, exercises the persistent memory subsystem and demonstrates the
enhanced tagging system with a short example.  It acts as an integration smoke
test and as a usage sample for developers.
"""

from __future__ import annotations

import sys
from pathlib import Path

# Ensure the project root is on ``sys.path`` so that ``src`` can be imported
ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.append(str(ROOT))

from src.core.neyra_brain import Neyra  # noqa: E402
from src.models import Character  # noqa: E402
from src.tags.enhanced_parser import EnhancedTagParser as TagParser  # noqa: E402
from src.tags.command_executor import CommandExecutor  # noqa: E402


def main() -> None:
    """Instantiate Neyra and exercise her core features."""

    neyra = Neyra()

    # 1) LLM usage ------------------------------------------------------------
    if neyra.llm is not None:
        try:
            greeting = neyra.llm.generate("Скажи краткое приветствие.", max_tokens=32)
            print(f"LLM: {greeting}")
        except Exception as exc:  # pragma: no cover - depends on heavy model
            print(f"LLM unavailable: {exc}")
            neyra.llm = None
    else:
        print("LLM: not configured")

    # 2) Memory subsystem -----------------------------------------------------
    alice = Character(name="Алиса", personality_traits=["смелая"])
    neyra.characters_memory.add(alice)
    neyra.characters_memory.save()
    print("Memory stored:", neyra.characters_memory.get("Алиса"))

    # 3) Tagging and command execution --------------------------------------
    parser = TagParser()
    executor = CommandExecutor(neyra)
    user_text = (
        "@Персонаж: Алиса - смелая@ "
        "@Эмоция: радость@ "
        "@Диалог: Привет, как дела?@ "
        "@Сцена: солнечный день в парке@"
    )
    tags = parser.parse_user_input(user_text)

    context: dict[str, str] = {}
    for tag in tags:
        result = executor.execute_command(tag, context)
        print(f"{tag.type}: {result}")
        if tag.type == "emotion_paint":
            context["emotion"] = tag.content


if __name__ == "__main__":  # pragma: no cover - manual invocation helper
    main()

