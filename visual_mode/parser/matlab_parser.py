from __future__ import annotations

"""MATLAB source parser for visual programming mode.

This parser performs a lightweight tokenization of MATLAB ``.m`` files to
extract top-level function and variable assignments.  Documentation metadata is
collected from line ``%`` comments or block ``%{`` ... ``%}`` comments.  The
resulting structure mirrors the format used by other language parsers and can
be consumed by the visual editor."""

from dataclasses import dataclass
import re
from pathlib import Path
from typing import Any, Dict, Iterable, List

from .base import LanguageParser


# ---------------------------------------------------------------------------
# Tokenization and comment handling
# ---------------------------------------------------------------------------

_TOKEN_RE = re.compile(r"[A-Za-z_][A-Za-z0-9_]*|==|~=|<=|>=|\d+\.\d+|\d+|\S")


def _tokenize_line(line: str) -> List[str]:
    """Return a list of tokens for a single line of MATLAB code."""

    code = line.split("%", 1)[0]
    return _TOKEN_RE.findall(code)


def _collect_comments(lines: List[str]) -> Dict[int, str]:
    """Map code line numbers to associated comments."""

    comments: Dict[int, str] = {}
    pending: List[str] = []
    i = 0
    while i < len(lines):
        line = lines[i]
        stripped = line.strip()

        # Block comments %{ ... %}
        if stripped.startswith("%{"):
            block = stripped[2:]
            if "%}" in block:
                block = block.split("%}", 1)[0]
                pending.append(block.strip())
                i += 1
                continue
            i += 1
            while i < len(lines):
                blk_line = lines[i]
                if "%}" in blk_line:
                    before_end = blk_line.split("%}", 1)[0]
                    pending.append(before_end.strip())
                    i += 1
                    break
                pending.append(blk_line.strip())
                i += 1
            continue

        # Full line comments starting with %
        if stripped.startswith("%"):
            pending.append(stripped[1:].strip())
            i += 1
            continue

        # Empty line resets pending comments
        if not stripped:
            pending.clear()
            i += 1
            continue

        # Inline comments
        inline = None
        if "%" in line:
            idx = line.find("%")
            if line[:idx].strip():
                inline = line[idx + 1 :].strip()
        if inline is not None:
            comments[i + 1] = inline
            pending.clear()
            i += 1
            continue

        # Associate pending comments with this line of code
        if pending:
            comments[i + 1] = " ".join(pending).strip()
            pending.clear()
        i += 1

    return comments


# ---------------------------------------------------------------------------
# Parsed representation
# ---------------------------------------------------------------------------


@dataclass
class ParsedMatlab:
    """Container holding parsed information about a MATLAB source file."""

    source: str
    lines: List[str]
    comments: Dict[int, str]


# ---------------------------------------------------------------------------
# Public parser implementation
# ---------------------------------------------------------------------------


class MatlabParser(LanguageParser):
    """Concrete :class:`LanguageParser` implementation for MATLAB."""

    _ASSIGN_RE = re.compile(r"^\s*([A-Za-z_][A-Za-z0-9_]*)\s*=\s*")

    def parse_file(self, path: str | Path) -> ParsedMatlab:
        source = Path(path).read_text(encoding="utf-8")
        lines = source.splitlines()
        comments = _collect_comments(lines)
        return ParsedMatlab(source=source, lines=lines, comments=comments)

    def _function_name(self, tokens: List[str]) -> str:
        if "=" in tokens:
            try:
                idx = tokens.index("=")
                return tokens[idx + 1]
            except Exception:  # pragma: no cover - defensive
                return ""
        if len(tokens) > 1:
            return tokens[1]
        return ""

    def extract_nodes(self, module: ParsedMatlab) -> Iterable[Dict[str, Any]]:
        nodes: List[Dict[str, Any]] = []
        in_function = 0
        for idx, line in enumerate(module.lines, start=1):
            tokens = _tokenize_line(line)
            if not tokens:
                continue
            first = tokens[0]
            if first == "function":
                name = self._function_name(tokens)
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
                in_function += 1
                continue
            if first == "end" and in_function > 0:
                in_function -= 1
                continue
            if in_function == 0:
                match = self._ASSIGN_RE.match(line)
                if match:
                    name = match.group(1)
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
        return nodes

    def extract_connections(self, module: ParsedMatlab) -> Iterable[Any]:
        return []
