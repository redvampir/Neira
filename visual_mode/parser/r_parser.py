from __future__ import annotations

"""R source parser for visual programming mode.

This parser uses a lightweight tokenizer implemented in Python to extract
top-level function and variable assignments from R source code. Documentation
is derived from inline ``#`` comments or preceding comment lines. The resulting
structure mirrors the format produced by other language parsers and can be
consumed by the visual editor."""

from dataclasses import dataclass
import re
from pathlib import Path
from typing import Any, Dict, Iterable, List

from .base import LanguageParser


@dataclass
class ParsedR:
    """Container holding parsed information about an R source file."""

    source: str
    lines: List[str]
    comments: Dict[int, str]


def _collect_comments(lines: List[str]) -> Dict[int, str]:
    """Map code line numbers to associated ``#`` comment text."""

    comments: Dict[int, str] = {}
    pending: List[str] = []
    for idx, line in enumerate(lines, start=1):
        stripped = line.strip()
        if stripped.startswith("#"):
            pending.append(stripped.lstrip("#").strip())
            continue
        if not stripped:
            pending.clear()
            continue
        doc = ""
        if "#" in line:
            doc = line.split("#", 1)[1].strip()
        elif pending:
            doc = " ".join(pending).strip()
        if doc:
            comments[idx] = doc
        pending.clear()
    return comments


class RParser(LanguageParser):
    """Concrete :class:`LanguageParser` implementation for R."""

    _FUNC_RE = re.compile(r"^\s*([A-Za-z\.][A-Za-z0-9\._]*)\s*(?:<-|=)\s*function\s*\(")
    _ASSIGN_RE = re.compile(r"^\s*([A-Za-z\.][A-Za-z0-9\._]*)\s*(?:<-|=)")

    def parse_file(self, path: str | Path) -> ParsedR:
        source = Path(path).read_text(encoding="utf-8")
        lines = source.splitlines()
        comments = _collect_comments(lines)
        return ParsedR(source=source, lines=lines, comments=comments)

    def extract_nodes(self, module: ParsedR) -> Iterable[Dict[str, Any]]:
        nodes: List[Dict[str, Any]] = []
        brace_level = 0
        for idx, line in enumerate(module.lines, start=1):
            code = line.split("#", 1)[0]
            if brace_level == 0:
                func_match = self._FUNC_RE.match(code)
                if func_match:
                    name = func_match.group(1)
                    doc = module.comments.get(idx, "")
                    nodes.append(
                        {
                            "id": name,
                            "type": "block",
                            "display": doc,
                            "range": {
                                "start": {"line": idx, "column": 1},
                                "end": {"line": idx, "column": len(line) + 1},
                            },
                        }
                    )
                else:
                    assign_match = self._ASSIGN_RE.match(code)
                    if assign_match:
                        name = assign_match.group(1)
                        doc = module.comments.get(idx, "")
                        nodes.append(
                            {
                                "id": name,
                                "type": "variable",
                                "display": doc,
                                "range": {
                                    "start": {"line": idx, "column": 1},
                                    "end": {"line": idx, "column": len(line) + 1},
                                },
                            }
                        )
            brace_level += code.count("{") - code.count("}")
        return nodes

    def extract_connections(self, module: ParsedR) -> Iterable[Any]:
        return []
