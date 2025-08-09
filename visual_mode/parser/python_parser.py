from __future__ import annotations

"""Python source parser for visual programming mode.

The parser extracts top level functions and variables from Python source code
and associates them with human readable descriptions derived from docstrings or
inline comments.  The resulting data structure can later be transformed into the
metadata format expected by the visual editor.
"""

from dataclasses import dataclass
import ast
import io
import tokenize
from pathlib import Path
from typing import Dict, Iterable, List, Any

from .base import LanguageParser


@dataclass
class ParsedModule:
    """Container holding parsed information about a Python module."""

    tree: ast.Module
    source: str
    comments: Dict[int, str]


def _node_range(node: ast.AST) -> Dict[str, Dict[str, int]]:
    """Return a dictionary describing the start and end position of ``node``."""

    start = {"line": getattr(node, "lineno", 1), "column": getattr(node, "col_offset", 0) + 1}
    end = {
        "line": getattr(node, "end_lineno", getattr(node, "lineno", 1)),
        "column": getattr(node, "end_col_offset", getattr(node, "col_offset", 0) + 1),
    }
    return {"start": start, "end": end}


class PythonParser(LanguageParser):
    """Concrete :class:`LanguageParser` implementation for Python."""

    def parse_file(self, path: str | Path) -> ParsedModule:
        source = Path(path).read_text(encoding="utf-8")
        tree = ast.parse(source, filename=str(path))

        # Map line numbers to inline comment text.
        comments: Dict[int, str] = {}
        for tok in tokenize.generate_tokens(io.StringIO(source).readline):
            if tok.type == tokenize.COMMENT:
                comments.setdefault(tok.start[0], tok.string.lstrip("# ").rstrip())

        return ParsedModule(tree=tree, source=source, comments=comments)

    def _variable_doc(self, module: ParsedModule, index: int, node: ast.stmt) -> str:
        """Return documentation string for a variable assignment ``node``."""

        # Inline comment on the same line takes precedence.
        doc = module.comments.get(node.lineno, "")
        if doc:
            return doc

        # A following string expression acts as a docstring annotation.
        body = module.tree.body
        if index + 1 < len(body):
            next_node = body[index + 1]
            if (
                isinstance(next_node, ast.Expr)
                and isinstance(getattr(next_node, "value", None), ast.Constant)
                and isinstance(next_node.value.value, str)
            ):
                return next_node.value.value.strip()
        return ""

    def extract_nodes(self, module: ParsedModule) -> Iterable[Dict[str, Any]]:
        nodes: List[Dict[str, Any]] = []
        body = module.tree.body
        for idx, node in enumerate(body):
            if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
                doc = ast.get_docstring(node) or ""
                nodes.append(
                    {
                        "id": node.name,
                        "type": "block",
                        "display": doc.strip(),
                        "range": _node_range(node),
                    }
                )
            elif isinstance(node, (ast.Assign, ast.AnnAssign)):
                targets: List[ast.Name] = []
                if isinstance(node, ast.Assign):
                    targets = [t for t in node.targets if isinstance(t, ast.Name)]
                else:  # AnnAssign
                    if isinstance(node.target, ast.Name):
                        targets = [node.target]
                if not targets:
                    continue
                doc = self._variable_doc(module, idx, node)
                for target in targets:
                    nodes.append(
                        {
                            "id": target.id,
                            "type": "variable",
                            "display": doc,
                            "range": _node_range(node),
                        }
                    )
        return nodes

    def extract_connections(self, module: ParsedModule) -> Iterable[Any]:
        return []
