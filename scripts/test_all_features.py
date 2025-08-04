
"""Run a small showcase of Neyra's capabilities.

The script exercises the local LLM, persistent memory, the tagging system and
action oriented features such as dialogue and scene generation. It is meant as
an integration smoke test and a usage example for developers.
"""

from __future__ import annotations

import json
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.append(str(ROOT))

from src.llm import MistralLLM  # noqa: E402
from src.memory import CharacterMemory  # noqa: E402
from src.models import Character  # noqa: E402
from src.tags.enhanced_parser import EnhancedTagParser as TagParser  # noqa: E402
from src.tags.command_executor import CommandExecutor  # noqa: E402


class Brain:
    """Minimal container replicating parts of Neyra's brain."""

    def __init__(self, llm: MistralLLM, max_tokens: int) -> None:
        self.llm = llm
        self.llm_max_tokens = max_tokens
        self.characters_memory = CharacterMemory()
        self.emotional_state = "нейтральная"


def main() -> None:
    config = json.loads((ROOT / "config" / "llm_config.json").read_text(encoding="utf-8"))
    model_path = config.get("model_path")
    max_tokens = int(config.get("max_tokens", 256))

    llm = MistralLLM(model_path)
    brain = Brain(llm, max_tokens)

    # 1) LLM usage ------------------------------------------------------------
    try:
        greeting = llm.generate("Скажи краткое приветствие.", max_tokens=32)
        print(f"LLM: {greeting}")
    except Exception as exc:  # pragma: no cover - depends on heavy model
        print(f"LLM unavailable: {exc}")
        brain.llm = None  # ensure fallbacks are used

    # 2) Memory subsystem -----------------------------------------------------
    alice = Character(name="Алиса", personality_traits=["смелая"])
    brain.characters_memory.add(alice)
    brain.characters_memory.save()
    print("Memory stored:", brain.characters_memory.get("Алиса"))

    # 3) Tagging and command execution --------------------------------------
    parser = TagParser()
    executor = CommandExecutor(brain)
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

