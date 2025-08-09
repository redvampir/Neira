from __future__ import annotations

"""C# source parser for visual programming mode.

This parser leverages Roslyn to parse C# source files and extracts method
and field declarations together with documentation from XML doc comments or
inline ``//`` annotations.
"""

from dataclasses import dataclass
from pathlib import Path
from typing import Any, Dict, Iterable, List
import xml.etree.ElementTree as ET

try:  # pragma: no cover - optional dependency
    import clr  # type: ignore
    clr.AddReference("Microsoft.CodeAnalysis")
    clr.AddReference("Microsoft.CodeAnalysis.CSharp")
    from Microsoft.CodeAnalysis.CSharp import CSharpSyntaxTree, SyntaxKind
    from Microsoft.CodeAnalysis.CSharp.Syntax import (
        FieldDeclarationSyntax,
        MethodDeclarationSyntax,
    )
except Exception:  # pragma: no cover - fallback when Roslyn is unavailable
    clr = None  # type: ignore
    CSharpSyntaxTree = None  # type: ignore
    SyntaxKind = None  # type: ignore
    MethodDeclarationSyntax = FieldDeclarationSyntax = object  # type: ignore

from .base import LanguageParser


@dataclass
class ParsedCSharp:
    """Container holding parsed information about a C# compilation unit."""

    tree: Any
    root: Any
    source: str


def _node_range(node: Any) -> Dict[str, Dict[str, int]]:
    """Return a dictionary describing the start and end position of ``node``."""

    span = node.GetLocation().GetLineSpan()
    start = span.StartLinePosition
    end = span.EndLinePosition
    return {
        "start": {"line": start.Line + 1, "column": start.Character + 1},
        "end": {"line": end.Line + 1, "column": end.Character + 1},
    }


def _xml_doc(node: Any) -> str:
    """Extract and flatten XML documentation for ``node``."""

    getter = getattr(node, "GetDocumentationCommentXml", None)
    if getter:
        xml = getter()
        if xml:
            try:
                root = ET.fromstring(xml)
                return " ".join(root.itertext()).strip()
            except Exception:  # pragma: no cover - malformed XML
                return xml.strip()
    return ""


def _inline_comment(node: Any) -> str:
    """Return trailing ``//`` comment associated with ``node`` if present."""

    if SyntaxKind is None:
        return ""
    token = node.GetLastToken()
    for trivia in token.TrailingTrivia:
        if trivia.Kind() == SyntaxKind.SingleLineCommentTrivia:
            return str(trivia.ToString()).lstrip("//").strip()
    return ""


class CSharpParser(LanguageParser):
    """Concrete :class:`LanguageParser` implementation for C#."""

    def parse_file(self, path: str | Path) -> ParsedCSharp:
        if CSharpSyntaxTree is None:  # pragma: no cover - dependency missing
            raise ImportError("Roslyn is required for CSharpParser")
        source = Path(path).read_text(encoding="utf-8")
        tree = CSharpSyntaxTree.ParseText(source)
        root = tree.GetRoot()
        return ParsedCSharp(tree=tree, root=root, source=source)

    def extract_nodes(self, module: ParsedCSharp) -> Iterable[Dict[str, Any]]:
        nodes: List[Dict[str, Any]] = []
        for node in module.root.DescendantNodes():
            if isinstance(node, MethodDeclarationSyntax):
                doc = _xml_doc(node) or _inline_comment(node)
                nodes.append(
                    {
                        "id": node.Identifier.Text,
                        "type": "block",
                        "display": doc,
                        "range": _node_range(node),
                    }
                )
            elif isinstance(node, FieldDeclarationSyntax):
                doc = _xml_doc(node) or _inline_comment(node)
                for variable in node.Declaration.Variables:
                    nodes.append(
                        {
                            "id": variable.Identifier.Text,
                            "type": "variable",
                            "display": doc,
                            "range": _node_range(node),
                        }
                    )
        return nodes

    def extract_connections(self, module: ParsedCSharp) -> Iterable[Any]:
        return []
