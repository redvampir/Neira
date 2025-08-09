"""Annotate a project with ``@neyra`` comments using translations.

The script walks through a given directory, adds the project root to
``sys.path`` so ``src`` modules become importable and applies
:class:`TranslationManager` to every ``.py`` file.
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

# Project root ---------------------------------------------------------------
ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.append(str(ROOT))

from src.translation.manager import TranslationManager  # noqa: E402


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Annotate Python files with @neyra comments")
    parser.add_argument("path", type=Path, help="Path to project to annotate")
    parser.add_argument(
        "--dictionary",
        type=Path,
        help="JSON file mapping identifiers to translations",
    )
    parser.add_argument("--lang", default="en", help="Target language code")
    return parser.parse_args()


def main() -> None:
    args = parse_args()

    dictionary: dict[str, str] = {}
    if args.dictionary and args.dictionary.exists():
        dictionary = json.loads(args.dictionary.read_text(encoding="utf-8"))

    manager = TranslationManager(dictionary)

    for file in Path(args.path).rglob("*.py"):
        source = file.read_text(encoding="utf-8")
        annotated = manager.annotate_source(source, args.lang)
        file.write_text(annotated, encoding="utf-8")


if __name__ == "__main__":  # pragma: no cover - CLI entry point
    main()
