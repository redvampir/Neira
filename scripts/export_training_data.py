"""Export recorded interactions for model training."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Iterable, Dict, Any

from src.learning.learning_system import LearningSystem


# ---------------------------------------------------------------------------
# Core functionality


def export_buffer(buffer: Iterable[Dict[str, Any]], dest: Path) -> int:
    """Write ``buffer`` entries to ``dest`` in JSONL format.

    Each item in ``buffer`` should be a mapping with ``request``, ``response``
    and ``rating`` keys. The resulting JSON lines will contain ``prompt`` and
    ``response`` fields expected by the training script.
    """

    dest.parent.mkdir(parents=True, exist_ok=True)
    count = 0
    with dest.open("w", encoding="utf-8") as fh:
        for item in buffer:
            entry = {
                "prompt": item.get("request", ""),
                "response": item.get("response", ""),
                "rating": item.get("rating", 0),
            }
            fh.write(json.dumps(entry, ensure_ascii=False) + "\n")
            count += 1
    return count


# ---------------------------------------------------------------------------
# Command line interface


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Export training data from a LearningSystem state file.",
    )
    parser.add_argument(
        "--state",
        type=Path,
        help="Path to a saved LearningSystem state JSON file.",
    )
    parser.add_argument(
        "--output",
        type=Path,
        default=Path("data/training/export.jsonl"),
        help="Destination JSONL file.",
    )
    return parser.parse_args()


def main() -> None:
    args = parse_args()

    if args.state:
        system = LearningSystem.load_state(args.state)
        buffer = system.experience_buffer
    else:
        try:
            from src.llm.manager import LLMManager

            manager = LLMManager()
            buffer = manager.learning_system.experience_buffer
        except Exception as exc:  # pragma: no cover - fallback for CLI
            raise RuntimeError(
                "No state file provided and unable to access live experience buffer"
            ) from exc

    count = export_buffer(buffer, args.output)
    print(f"Exported {count} interactions to {args.output}")


if __name__ == "__main__":
    main()
