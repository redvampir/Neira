from __future__ import annotations

"""Swift source parser for visual programming mode.

This parser relies on the `sourcekitten` command line tool to obtain a
structured representation of Swift source files.  It extracts top level
functions and type declarations together with their accompanying
``///`` or block ``/* ... */`` comments.  The resulting information mirrors
that of the other language parsers in this package and can be consumed by the
visual editor.
"""

from dataclasses import dataclass
import json
import re
import subprocess
from pathlib import Path
from typing import Any, Dict, Iterable, List

from .base import LanguageParser
from . import utils


@dataclass
class ParsedSwift:
    """Container holding parsed information about a Swift file."""

    structure: Dict[str, Any]
    source: str
    comments: Dict[int, str]


def _clean_comment_text(text: str) -> str:
    """Normalize comment text by stripping decorations."""

    lines = text.splitlines()
    cleaned: List[str] = []
    for line in lines:
        line = line.strip()
        line = line.lstrip("/").lstrip("*")
        cleaned.append(line.strip())
    return "\n".join([ln for ln in cleaned if ln]).strip()


def _extract_block_comments(source: str) -> Dict[int, str]:
    """Return mapping of line numbers to preceding block comments."""

    comments: Dict[int, str] = {}
    lines = source.splitlines()

    for match in re.finditer(r"/\*(.*?)\*/", source, re.DOTALL):
        comment_body = match.group(1)
        comment = _clean_comment_text(comment_body)

        end_offset = match.end()
        end_line = source.count("\n", 0, end_offset) + 1

        next_line = end_line + 1
        while next_line <= len(lines):
            text = lines[next_line - 1].strip()
            if text and not text.startswith("//") and not text.startswith("/*"):
                comments[next_line] = comment
                break
            next_line += 1

    return comments


def _node_range(item: Dict[str, Any], source: str) -> Dict[str, Dict[str, int]]:
    start = item.get("key.offset", 0)
    end = start + item.get("key.length", 0)
    s_line, s_col = utils.offset_to_position(source, start)
    e_line, e_col = utils.offset_to_position(source, end)
    return {"start": {"line": s_line, "column": s_col}, "end": {"line": e_line, "column": e_col}}


class SwiftParser(LanguageParser):
    """Concrete :class:`LanguageParser` implementation for Swift."""

    def parse_file(self, path: str | Path) -> ParsedSwift:
        path = Path(path)
        source = path.read_text(encoding="utf-8")
        proc = subprocess.run(
            ["sourcekitten", "structure", "--file", str(path)],
            check=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
        )
        structure: Dict[str, Any] = json.loads(proc.stdout or "{}")
        comments = _extract_block_comments(source)
        return ParsedSwift(structure=structure, source=source, comments=comments)

    def _docstring(self, item: Dict[str, Any], module: ParsedSwift) -> str:
        if "key.docoffset" in item and "key.doclength" in item:
            start = item["key.docoffset"]
            end = start + item["key.doclength"]
            raw = module.source[start:end]
            return _clean_comment_text(raw)
        line, _ = utils.offset_to_position(module.source, item.get("key.offset", 0))
        return module.comments.get(line, "")

    def _base_name(self, name: str) -> str:
        return name.split("(")[0]

    def _walk(self, items: List[Dict[str, Any]], module: ParsedSwift, nodes: List[Dict[str, Any]]) -> None:
        for item in items:
            kind = item.get("key.kind", "")
            name = item.get("key.name")
            sub = item.get("key.substructure", [])
            if name:
                base_name = self._base_name(name)
                if kind.startswith("source.lang.swift.decl.function"):
                    nodes.append(
                        {
                            "id": base_name,
                            "type": "block",
                            "display": self._docstring(item, module),
                            "range": _node_range(item, module.source),
                        }
                    )
                elif kind in {
                    "source.lang.swift.decl.struct",
                    "source.lang.swift.decl.class",
                    "source.lang.swift.decl.enum",
                }:
                    nodes.append(
                        {
                            "id": base_name,
                            "type": "block",
                            "display": self._docstring(item, module),
                            "range": _node_range(item, module.source),
                        }
                    )
            if sub:
                self._walk(sub, module, nodes)

    def extract_nodes(self, module: ParsedSwift) -> Iterable[Dict[str, Any]]:
        nodes: List[Dict[str, Any]] = []
        root = module.structure.get("key.substructure", [])
        self._walk(root, module, nodes)
        return nodes

    def extract_connections(self, module: ParsedSwift) -> Iterable[Any]:
        return []
