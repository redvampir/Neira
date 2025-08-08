"""Simple benchmarks comparing Python and Rust tag parsers."""

from __future__ import annotations

from dataclasses import dataclass
from timeit import timeit
from typing import List

from src.interaction.tag_processor import TagProcessor, ProcessedTag


@dataclass
class PyTag:
    type: str
    subject: str
    commands: List[str]


def python_parse(text: str) -> List[PyTag]:
    def render(tag: PyTag) -> str:
        base = f"@{tag.type.capitalize()}: {tag.subject}"
        if tag.commands:
            base += " " + " ".join(f"/{c}" for c in tag.commands)
        return base + "@"

    def parse_tag(segment: str, start: int):
        i = start + 1
        colon = segment.find(":", i)
        if colon == -1:
            return PyTag("", "", []), [], 1
        tag_type = segment[i:colon].strip().lower()
        i = colon + 1
        content_chars: List[str] = []
        inner: List[PyTag] = []
        while i < len(segment):
            ch = segment[i]
            if ch == "@":
                if i + 1 < len(segment) and segment[i + 1] == "@":
                    content_chars.append("@")
                    i += 2
                    continue
                next_at = segment.find("@", i + 1)
                next_colon = segment.find(":", i + 1)
                if next_colon != -1 and (next_at == -1 or next_colon < next_at):
                    nested_tag, nested_inner, consumed = parse_tag(segment, i)
                    inner.extend(nested_inner)
                    inner.append(nested_tag)
                    content_chars.append(render(nested_tag))
                    i += consumed
                    continue
                i += 1
                break
            else:
                content_chars.append(ch)
                i += 1
        content = "".join(content_chars).replace("@@", "@")
        parts = [p.strip() for p in content.split("/") if p.strip()]
        first = parts[0] if parts else ""
        commands: List[str] = []
        subject = first
        dash_split = "—" if "—" in first else "-" if "-" in first else None
        if dash_split:
            subject, cmd = [p.strip() for p in first.split(dash_split, 1)]
            if cmd:
                commands.append(cmd)
        for p in parts[1:]:
            commands.append(p)
        tag = PyTag(tag_type, subject, commands)
        return tag, inner, i - start

    tags: List[PyTag] = []
    i = 0
    while i < len(text):
        if text[i] == "@":
            tag, inner_tags, consumed = parse_tag(text, i)
            tags.extend(inner_tags)
            if tag.type:
                tags.append(tag)
            i += consumed
        else:
            i += 1
    return tags


SAMPLE_TEXT = "@Персонаж: Лили /внешность /стиль@" * 50


def main() -> None:
    rust_processor = TagProcessor()
    py_time = timeit(lambda: python_parse(SAMPLE_TEXT), number=200)
    rust_time = timeit(lambda: rust_processor.parse(SAMPLE_TEXT), number=200)
    print(f"Python parser: {py_time:.4f}s for 200 runs")
    print(f"Rust parser:   {rust_time:.4f}s for 200 runs")


if __name__ == "__main__":  # pragma: no cover - manual benchmark
    main()

