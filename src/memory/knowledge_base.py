"""Utilities for building a simple knowledge base from books.

This module analyses plain text books and extracts lightweight information
about characters, locations and author style.  The results are stored as JSON
files inside ``data/knowledge_base`` so that other components of the project
can use the collected knowledge.

The implementation is intentionally heuristic – it is designed only to support
tests and to serve as an example of how the system could evolve.  The goal of
the kata is not to provide a full featured NLP pipeline, but to have a
structured place where such logic could live.
"""

from __future__ import annotations

import json
import re
from pathlib import Path
from typing import Dict, List

# Root directory for knowledge base artefacts
KB_ROOT = Path("data/knowledge_base")


def _split_chapters(text: str) -> Dict[str, str]:
    """Split raw book text into chapters.

    Chapters are detected by the marker ``Глава <num>`` which is common for
    Russian texts.  The function returns an ordered dictionary mapping the
    chapter title to its body.
    """

    chapter_pattern = re.compile(r"Глава\s+\d+", re.IGNORECASE)
    parts = chapter_pattern.split(text)
    headers = chapter_pattern.findall(text)
    chapters: Dict[str, str] = {}
    for idx, body in enumerate(parts[1:], start=0):
        title = headers[idx] if idx < len(headers) else f"Глава {idx+1}"
        chapters[title.strip()] = body.strip()
    return chapters


def _extract_style_examples(text: str) -> List[str]:
    """Find blocks wrapped in special style markers.

    The markup looks like::

        [Пример стиля автора, из главы 1]
        ... текст ...
        [Пример окончен]
    """

    pattern = re.compile(
        r"\[Пример стиля автора,.*?\](.*?)\[Пример окончен\]",
        re.DOTALL,
    )
    examples = [match.strip() for match in pattern.findall(text)]
    return examples


def analyze_book(file_path: str) -> Dict[str, Dict[str, str]]:
    """Analyse a book and populate the knowledge base.

    Parameters
    ----------
    file_path:
        Path to a text file containing the book.

    Returns
    -------
    Dict[str, Dict[str, str]]
        A dictionary with extracted information which mirrors the JSON files
        written to disk.
    """

    path = Path(file_path)
    if not path.exists():
        raise FileNotFoundError(f"Book file not found: {file_path}")
    try:
        text = path.read_text(encoding="utf-8")
    except Exception as exc:
        raise RuntimeError(f"Failed to read book file '{file_path}': {exc}") from exc

    KB_ROOT.mkdir(parents=True, exist_ok=True)

    chapters = _split_chapters(text)
    style_examples = _extract_style_examples(text)

    # Very naive character extraction: take all capitalised words
    char_pattern = re.compile(r"\b[А-ЯЁ][а-яё]+\b")
    characters: Dict[str, Dict[str, str]] = {}

    for chapter_name, body in chapters.items():
        for name in set(char_pattern.findall(body)):
            key = name.lower()
            if key not in characters:
                characters[key] = {
                    "id": key,
                    "name": name,
                    "appearance": "",
                    "first_appearance": chapter_name,
                    "speech_style": "",
                    "tags": [],
                }

    # Locations are not extracted yet; the structure is kept for future work
    locations: Dict[str, Dict[str, str]] = {}

    # Persist data to JSON files so other modules can reuse it
    (KB_ROOT / "characters.json").write_text(
        json.dumps(characters, ensure_ascii=False, indent=2),
        encoding="utf-8",
    )
    (KB_ROOT / "locations.json").write_text(
        json.dumps(locations, ensure_ascii=False, indent=2),
        encoding="utf-8",
    )
    (KB_ROOT / "style.json").write_text(
        json.dumps({"examples": style_examples}, ensure_ascii=False, indent=2),
        encoding="utf-8",
    )
    index = {
        "characters": [c["name"] for c in characters.values()],
        "locations": [l["name"] for l in locations.values()],
        "style_examples": len(style_examples),
    }
    (KB_ROOT / "index.json").write_text(
        json.dumps(index, ensure_ascii=False, indent=2),
        encoding="utf-8",
    )

    return {
        "characters": characters,
        "locations": locations,
        "style_examples": style_examples,
        "index": index,
    }


__all__ = ["analyze_book"]

