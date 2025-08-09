from __future__ import annotations

"""JavaScript source parser for visual programming mode.

This module uses the :mod:`esprima` package to parse JavaScript source code and
extract top level function and variable declarations together with any
preceding or inline comments.  Both ``//`` and ``/* ... */`` comment styles are
supported, including JSDoc style comments.  The extracted information mirrors
that of other language parsers in this package and can be consumed by the
visual editor.
"""

from dataclasses import dataclass
from pathlib import Path
from typing import Any, Dict, Iterable, List

import esprima

from .base import LanguageParser


@dataclass
class ParsedJavaScript:
    """Container holding parsed information about a JavaScript module."""

    tree: Any
    source: str
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


def _extract_comments(source: str) -> Dict[int, str]:
    """Return mapping of line numbers to comments for the following code line."""

    comments: Dict[int, str] = {}
    lines = source.splitlines()
    pending: List[str] = []
    i = 0
    while i < len(lines):
        line = lines[i]
        stripped = line.strip()

        # Handle start of block comments (including JSDoc)
        if stripped.startswith("/*"):
            start_line = line
            start = start_line.find("/*")
            block = start_line[start + 2 :]
            end = start_line.find("*/", start + 2)
            current_line = start_line
            while end == -1 and i + 1 < len(lines):
                i += 1
                current_line = lines[i]
                block += "\n" + current_line
                end = current_line.find("*/")
            if end != -1:
                block_content = block[: block.rfind("*/")]
                remainder = current_line[end + 2 :]
            else:
                block_content = block
                remainder = ""
            cleaned = _clean_comment_text(block_content)
            before = start_line[:start]
            if before.strip() or remainder.strip():
                comments[i + 1] = cleaned
                pending = []
            else:
                pending.append(cleaned)
            i += 1
            continue

        # Accumulate full line // comments
        if stripped.startswith("//"):
            pending.append(stripped[2:].strip())
            i += 1
            continue

        inline_comment: str | None = None
        if "//" in line:
            idx = line.find("//")
            if line[:idx].strip():
                inline_comment = line[idx + 2 :].strip()
        if inline_comment is None and "/*" in line and "*/" in line and line.find("/*") > 0:
            start = line.find("/*")
            end = line.find("*/", start + 2)
            if line[:start].strip():
                inline_comment = _clean_comment_text(line[start + 2 : end])

        if inline_comment is not None:
            comments[i + 1] = inline_comment
            pending = []
            i += 1
            continue

        if stripped:
            if pending:
                comments[i + 1] = "\n".join(pending).strip()
                pending = []
        i += 1
    return comments


def _node_range(node: Any) -> Dict[str, Dict[str, int]]:
    """Return a dictionary describing the start and end position of ``node``."""

    loc = getattr(node, "loc", None)
    if loc is None:
        start = {"line": 1, "column": 1}
        end = start
    else:
        start = {"line": loc.start.line, "column": loc.start.column + 1}
        end = {"line": loc.end.line, "column": loc.end.column + 1}
    return {"start": start, "end": end}


class JavaScriptParser(LanguageParser):
    """Concrete :class:`LanguageParser` implementation for JavaScript."""

    def parse_file(self, path: str | Path) -> ParsedJavaScript:
        source = Path(path).read_text(encoding="utf-8")
        tree = esprima.parseScript(source, loc=True)
        comments = _extract_comments(source)
        return ParsedJavaScript(tree=tree, source=source, comments=comments)

    def extract_nodes(self, module: ParsedJavaScript) -> Iterable[Dict[str, Any]]:
        nodes: List[Dict[str, Any]] = []
        for stmt in module.tree.body:
            if stmt.type == "FunctionDeclaration":
                name = stmt.id.name if stmt.id else ""
                doc = module.comments.get(stmt.loc.start.line, "")
                nodes.append(
                    {
                        "id": name,
                        "type": "block",
                        "display": doc,
                        "range": _node_range(stmt),
                    }
                )
            elif stmt.type == "VariableDeclaration":
                for decl in stmt.declarations:
                    if decl.id.type != "Identifier":
                        continue
                    name = decl.id.name
                    doc = module.comments.get(decl.loc.start.line, module.comments.get(stmt.loc.start.line, ""))
                    nodes.append(
                        {
                            "id": name,
                            "type": "variable",
                            "display": doc,
                            "range": _node_range(decl),
                        }
                    )
        return nodes

    def extract_connections(self, module: ParsedJavaScript) -> Iterable[Any]:
        return []
