from __future__ import annotations

"""Java source parser for visual programming mode.

This module provides :class:`JavaParser` which converts Java source code into a
structure of visual blocks.  It uses the :mod:`javalang` library to parse the
source into an AST and extracts documentation from block comments and
Javadoc-style annotations.
"""

from dataclasses import dataclass
from pathlib import Path
from typing import Any, Dict, Iterable, List
import re

import javalang

from .base import LanguageParser


@dataclass
class ParsedJava:
    """Container holding parsed information about a Java compilation unit."""

    tree: javalang.tree.CompilationUnit
    source: str
    comments: Dict[int, str]


def _clean_comment_text(text: str) -> str:
    """Normalize comment text by stripping decorations."""

    lines = text.splitlines()
    cleaned = []
    for line in lines:
        line = line.strip()
        if line.startswith("*"):
            line = line.lstrip("*")
        cleaned.append(line.strip())
    return "\n".join([ln for ln in cleaned if ln]).strip()


def _extract_block_comments(source: str) -> Dict[int, str]:
    """Return mapping of line numbers to preceding block comments.

    The returned dictionary maps the *first code line* following a block or
    Javadoc comment to the comment's text.
    """

    comments: Dict[int, str] = {}
    lines = source.splitlines()

    for match in re.finditer(r"/\*+(.*?)\*/", source, re.DOTALL):
        comment_body = match.group(1)
        comment = _clean_comment_text(comment_body)

        # Determine the line where the comment ends
        end_offset = match.end()
        end_line = source[:end_offset].count("\n") + 1

        # Find the next line containing code
        next_line = end_line + 1
        while next_line <= len(lines):
            text = lines[next_line - 1].strip()
            if text and not text.startswith("//") and not text.startswith("/*"):
                comments[next_line] = comment
                break
            next_line += 1

    return comments


def _node_range(node: Any) -> Dict[str, Dict[str, int]]:
    """Return a dictionary describing the start and end position of ``node``."""

    if getattr(node, "position", None):
        line, col = node.position
    else:
        line, col = 1, 0
    start = {"line": line, "column": col + 1}
    return {"start": start, "end": start}


class JavaParser(LanguageParser):
    """Concrete :class:`LanguageParser` implementation for Java."""

    def parse_file(self, path: str | Path) -> ParsedJava:
        source = Path(path).read_text(encoding="utf-8")
        tree = javalang.parse.parse(source)
        comments = _extract_block_comments(source)
        return ParsedJava(tree=tree, source=source, comments=comments)

    def extract_nodes(self, module: ParsedJava) -> Iterable[Dict[str, Any]]:
        nodes: List[Dict[str, Any]] = []
        tree = module.tree

        # Package declaration
        if tree.package:
            pkg = tree.package
            line = pkg.position.line if pkg.position else 1
            doc = module.comments.get(line, "")
            nodes.append(
                {
                    "id": pkg.name,
                    "type": "block",
                    "display": doc,
                    "range": _node_range(pkg),
                }
            )

        # Classes and their methods/constructors
        for _, class_node in tree.filter(javalang.tree.ClassDeclaration):
            line = class_node.position.line if class_node.position else 1
            doc = module.comments.get(line, "")
            nodes.append(
                {
                    "id": class_node.name,
                    "type": "block",
                    "display": doc,
                    "range": _node_range(class_node),
                }
            )
            # Methods
            for method in list(class_node.methods) + list(class_node.constructors):
                line = method.position.line if method.position else 1
                doc = module.comments.get(line, "")
                nodes.append(
                    {
                        "id": method.name,
                        "type": "block",
                        "display": doc,
                        "range": _node_range(method),
                    }
                )

        return nodes

    def extract_connections(self, module: ParsedJava) -> Iterable[Any]:
        return []
