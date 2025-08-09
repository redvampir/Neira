from __future__ import annotations

"""Objective-C source parser for visual programming mode.

This parser relies on clang's Python bindings to process Objective-C header
and implementation files.  It extracts class/interface declarations and their
methods, associating them with preceding ``//`` or ``/* ... */`` comments which
serve as annotations for the visual mode."""

from dataclasses import dataclass
from pathlib import Path
from typing import Any, Dict, Iterable, List
from ctypes.util import find_library

from clang import cindex

# Attempt to locate libclang dynamically.  The tests set up the environment so
# that this library can be found.  We try :func:`find_library` first and then
# fall back to common installation locations for recent LLVM versions.
libclang_path = find_library("clang")
if not libclang_path:
    for ver in range(20, 11, -1):  # search a range of typical versions
        candidate = Path(f"/usr/lib/llvm-{ver}/lib/libclang.so")
        if candidate.exists():
            libclang_path = str(candidate)
            break
if libclang_path:
    try:  # pragma: no cover - environment dependent
        cindex.Config.set_library_file(libclang_path)
    except Exception:
        pass

from .base import LanguageParser
from .c_parser import _extract_block_comments, _node_range


@dataclass
class ParsedObjC:
    """Container holding parsed information about an Objective-C file."""

    translation_unit: cindex.TranslationUnit
    source: str
    path: Path
    comments: Dict[int, str]


def _extract_comments(source: str) -> Dict[int, str]:
    """Return mapping of line numbers to preceding comments.

    Both block (``/* ... */``) and single line (``//``) comments are supported.
    The returned dictionary maps the first code line following a comment block
    to the cleaned comment text.
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


class ObjCParser(LanguageParser):
    """Concrete :class:`LanguageParser` implementation for Objective-C."""

    def parse_file(self, path: str | Path) -> ParsedObjC:
        path = Path(path)
        source = path.read_text(encoding="utf-8")
        index = cindex.Index.create()
        options = cindex.TranslationUnit.PARSE_DETAILED_PROCESSING_RECORD
        lang = "objective-c-header" if path.suffix == ".h" else "objective-c"
        args = ["-x", lang, f"-I{path.parent}"]
        tu = index.parse(str(path), args=args, options=options)
        comments = _extract_comments(source)
        return ParsedObjC(translation_unit=tu, source=source, path=path, comments=comments)

    def extract_nodes(self, module: ParsedObjC) -> Iterable[Dict[str, Any]]:
        nodes: List[Dict[str, Any]] = []
        path = module.path

        def walk(cursor: cindex.Cursor) -> None:
            for child in cursor.get_children():
                loc = child.location
                if loc.file is None or Path(loc.file.name) != path:
                    continue
                kind = child.kind
                if kind in (
                    cindex.CursorKind.OBJC_INTERFACE_DECL,
                    cindex.CursorKind.OBJC_IMPLEMENTATION_DECL,
                    cindex.CursorKind.OBJC_CATEGORY_DECL,
                    cindex.CursorKind.OBJC_PROTOCOL_DECL,
                ):
                    doc = module.comments.get(child.extent.start.line, "")
                    nodes.append(
                        {
                            "id": child.spelling,
                            "type": "block",
                            "display": doc,
                            "range": _node_range(child),
                        }
                    )
                    walk(child)
                elif kind in (
                    cindex.CursorKind.OBJC_INSTANCE_METHOD_DECL,
                    cindex.CursorKind.OBJC_CLASS_METHOD_DECL,
                    cindex.CursorKind.OBJC_PROPERTY_DECL,
                    cindex.CursorKind.FUNCTION_DECL,
                ):
                    doc = module.comments.get(child.extent.start.line, "")
                    nodes.append(
                        {
                            "id": child.spelling,
                            "type": "block",
                            "display": doc,
                            "range": _node_range(child),
                        }
                    )
                else:
                    walk(child)

        walk(module.translation_unit.cursor)
        return nodes

    def extract_connections(self, module: ParsedObjC) -> Iterable[Any]:
        return []
