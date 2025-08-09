from __future__ import annotations

"""C++ source parser for visual programming mode.

Extends :class:`CParser` with support for namespaces, templates and line
comments (``//``) in addition to block comments for annotations.
"""

from pathlib import Path
from typing import Any, Dict, Iterable, List

from clang import cindex

from .c_parser import CParser, ParsedC, _extract_block_comments, _node_range


def _extract_comments(source: str) -> Dict[int, str]:
    """Return mapping of line numbers to preceding comments.

    Both block (``/* ... */``) and single line (``//``) comments are supported.
    """

    comments = _extract_block_comments(source)
    lines = source.splitlines()
    i = 0
    while i < len(lines):
        stripped = lines[i].strip()
        if stripped.startswith("//"):
            block: List[str] = []
            while i < len(lines) and lines[i].strip().startswith("//"):
                block.append(lines[i].strip()[2:].strip())
                i += 1
            while i < len(lines) and not lines[i].strip():
                i += 1
            if i < len(lines):
                comments[i + 1] = "\n".join([ln for ln in block if ln]).strip()
            continue
        i += 1
    return comments


class CppParser(CParser):
    """Concrete :class:`LanguageParser` implementation for C++."""

    def parse_file(self, path: str | Path) -> ParsedC:
        path = Path(path)
        source = path.read_text(encoding="utf-8")
        index = cindex.Index.create()
        options = cindex.TranslationUnit.PARSE_DETAILED_PROCESSING_RECORD
        args = ["-std=c++17", "-x", "c++"]
        tu = index.parse(str(path), args=args, options=options)
        comments = _extract_comments(source)
        return ParsedC(translation_unit=tu, source=source, path=path, comments=comments)

    def extract_nodes(self, module: ParsedC) -> Iterable[Dict[str, Any]]:
        nodes: List[Dict[str, Any]] = []
        path = module.path

        def walk(cursor: cindex.Cursor, namespace: str = "") -> None:
            for child in cursor.get_children():
                loc = child.location
                if loc.file is None or Path(loc.file.name) != path:
                    continue
                if child.kind == cindex.CursorKind.NAMESPACE:
                    ns = f"{namespace}::{child.spelling}" if namespace else child.spelling
                    walk(child, ns)
                elif child.kind in (
                    cindex.CursorKind.FUNCTION_DECL,
                    cindex.CursorKind.FUNCTION_TEMPLATE,
                ):
                    doc = module.comments.get(child.extent.start.line, "")
                    name = child.spelling
                    if namespace:
                        name = f"{namespace}::{name}"
                    nodes.append(
                        {
                            "id": name,
                            "type": "block",
                            "display": doc,
                            "range": _node_range(child),
                        }
                    )
                elif child.kind == cindex.CursorKind.MACRO_DEFINITION:
                    doc = module.comments.get(child.extent.start.line, "")
                    name = child.spelling
                    if namespace:
                        name = f"{namespace}::{name}"
                    nodes.append(
                        {
                            "id": name,
                            "type": "macro",
                            "display": doc,
                            "range": _node_range(child),
                        }
                    )
                else:
                    walk(child, namespace)

        walk(module.translation_unit.cursor)
        return nodes

    def extract_connections(self, module: ParsedC) -> Iterable[Any]:
        return []
