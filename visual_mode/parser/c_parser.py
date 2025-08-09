from __future__ import annotations

"""C source parser for visual programming mode.

This module uses libclang's Python bindings to parse C source and header files,
extracting functions and macros together with documentation provided via block
comments (``/* ... */``).  The resulting data structure mirrors the format used
by other language parsers in this package.
"""

from dataclasses import dataclass
from pathlib import Path
from typing import Any, Dict, Iterable, List
import re
from ctypes.util import find_library

from clang import cindex

# Attempt to locate libclang dynamically.  Tests configure the environment so
# that the library can be found.  Fallbacks are intentionally permissive to
# avoid import errors when the library is unavailable.
libclang_path = find_library("clang") or "/usr/lib/llvm-18/lib/libclang.so"
if libclang_path:
    try:  # pragma: no cover - environment dependent
        cindex.Config.set_library_file(libclang_path)
    except Exception:
        pass

from .base import LanguageParser


@dataclass
class ParsedC:
    """Container holding parsed information about a C translation unit."""

    translation_unit: cindex.TranslationUnit
    source: str
    path: Path
    comments: Dict[int, str]


def _clean_comment_text(text: str) -> str:
    """Normalize comment text by stripping decorations."""

    lines = text.splitlines()
    cleaned: List[str] = []
    for line in lines:
        line = line.strip()
        if line.startswith("*"):
            line = line.lstrip("*")
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
        end_line = source[:end_offset].count("\n") + 1

        next_line = end_line + 1
        while next_line <= len(lines):
            text = lines[next_line - 1].strip()
            if text and not text.startswith("//") and not text.startswith("/*"):
                comments[next_line] = comment
                break
            next_line += 1

    return comments


def _node_range(cursor: cindex.Cursor) -> Dict[str, Dict[str, int]]:
    """Return a dictionary describing the start and end position of ``cursor``."""

    start = {"line": cursor.extent.start.line, "column": cursor.extent.start.column}
    end = {"line": cursor.extent.end.line, "column": cursor.extent.end.column}
    return {"start": start, "end": end}


class CParser(LanguageParser):
    """Concrete :class:`LanguageParser` implementation for C."""

    def parse_file(self, path: str | Path) -> ParsedC:
        path = Path(path)
        source = path.read_text(encoding="utf-8")
        index = cindex.Index.create()
        options = cindex.TranslationUnit.PARSE_DETAILED_PROCESSING_RECORD
        args = ["-std=c11", "-x", "c"]
        tu = index.parse(str(path), args=args, options=options)
        comments = _extract_block_comments(source)
        return ParsedC(translation_unit=tu, source=source, path=path, comments=comments)

    def extract_nodes(self, module: ParsedC) -> Iterable[Dict[str, Any]]:
        nodes: List[Dict[str, Any]] = []
        path = module.path
        for cursor in module.translation_unit.cursor.get_children():
            loc = cursor.location
            if loc.file is None or Path(loc.file.name) != path:
                continue
            if cursor.kind == cindex.CursorKind.FUNCTION_DECL:
                doc = module.comments.get(cursor.extent.start.line, "")
                nodes.append(
                    {
                        "id": cursor.spelling,
                        "type": "block",
                        "display": doc,
                        "range": _node_range(cursor),
                    }
                )
            elif cursor.kind == cindex.CursorKind.MACRO_DEFINITION:
                doc = module.comments.get(cursor.extent.start.line, "")
                nodes.append(
                    {
                        "id": cursor.spelling,
                        "type": "macro",
                        "display": doc,
                        "range": _node_range(cursor),
                    }
                )
        return nodes

    def extract_connections(self, module: ParsedC) -> Iterable[Any]:
        return []
