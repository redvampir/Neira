from __future__ import annotations

"""Dart source parser for visual programming mode.

This parser delegates Dart syntax analysis to a small Dart program that uses the
`analyzer` package.  The program extracts top level function, class and variable
metadata together with their documentation comments.  Line ``///`` comments as
well as block ``/* ... */`` comments are considered for metadata.  The resulting
information mirrors that of the other language parsers in this package and can
be consumed by the visual editor.
"""

from dataclasses import dataclass
import json
import re
import shutil
import subprocess
import tempfile
from pathlib import Path
from typing import Any, Dict, Iterable, List

from .base import LanguageParser

DART_PARSER_SOURCE = r"""
import 'dart:convert';
import 'dart:io';
import 'package:analyzer/dart/analysis/utilities.dart';

String cleanDoc(Comment? comment) {
  if (comment == null) return '';
  return comment.tokens
      .map((t) {
        var text = t.lexeme;
        if (text.startsWith('///')) {
          text = text.substring(3);
        } else {
          text = text.replaceFirst('/*', '').replaceFirst('*/', '');
        }
        return text
            .split('\n')
            .map((l) => l.trim().replaceFirst('*', '').trim())
            .join('\n');
      })
      .join('\n')
      .trim();
}

Map<String, dynamic> range(LineInfo info, AstNode node) {
  final start = info.getLocation(node.offset);
  final end = info.getLocation(node.end);
  return {
    'start': {'line': start.lineNumber, 'column': start.columnNumber},
    'end': {'line': end.lineNumber, 'column': end.columnNumber},
  };
}

void main(List<String> args) {
  if (args.isEmpty) return;
  final path = args[0];
  final source = File(path).readAsStringSync();
  final result = parseString(content: source, path: path);
  final unit = result.unit;
  final info = unit.lineInfo;
  final nodes = <Map<String, dynamic>>[];

  for (final decl in unit.declarations) {
    if (decl is FunctionDeclaration) {
      final name = decl.name.lexeme;
      final doc = cleanDoc(decl.documentationComment);
      nodes.add({
        'kind': 'function',
        'name': name,
        'doc': doc,
        'range': range(info, decl),
      });
    } else if (decl is ClassDeclaration) {
      final name = decl.name.lexeme;
      final doc = cleanDoc(decl.documentationComment);
      nodes.add({
        'kind': 'class',
        'name': name,
        'doc': doc,
        'range': range(info, decl),
      });
    } else if (decl is TopLevelVariableDeclaration) {
      final doc = cleanDoc(decl.documentationComment);
      for (final v in decl.variables.variables) {
        final name = v.name.lexeme;
        nodes.add({
          'kind': 'variable',
          'name': name,
          'doc': doc,
          'range': range(info, v),
        });
      }
    }
  }

  stdout.write(jsonEncode({'nodes': nodes}));
}
"""


@dataclass
class ParsedDart:
    """Container holding parsed information about a Dart compilation unit."""

    nodes: List[Dict[str, Any]]


def _clean_comment_text(text: str) -> str:
    """Normalize block comment ``text`` by stripping decorations."""

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

    for match in re.finditer(r"/\*(?!\*)(.*?)\*/", source, re.DOTALL):
        body = match.group(1)
        comment = _clean_comment_text(body)

        end_offset = match.end()
        end_line = source.count("\n", 0, end_offset) + 1

        next_line = end_line + 1
        while next_line <= len(lines):
            text = lines[next_line - 1].trim()
            if text and not text.startswith("//") and not text.startswith("/*"):
                comments[next_line] = comment
                break
            next_line += 1

    return comments


class DartParser(LanguageParser):
    """Concrete :class:`LanguageParser` implementation for Dart."""

    def _ensure_dart(self) -> str:
        dart = shutil.which("dart")
        if dart is None:
            raise EnvironmentError("dart executable not found")
        return dart

    def parse_file(self, path: str | Path) -> ParsedDart:
        path = Path(path)
        source = path.read_text(encoding="utf-8")
        dart = self._ensure_dart()
        with tempfile.TemporaryDirectory() as tmpdir:
            tmp = Path(tmpdir)
            (tmp / "pubspec.yaml").write_text(
                "name: dart_parser\nversion: 1.0.0\nenvironment:\n  sdk: '>=3.0.0 <4.0.0'\ndependencies:\n  analyzer: ^6.0.0\n",
                encoding="utf-8",
            )
            (tmp / "bin").mkdir()
            (tmp / "bin" / "main.dart").write_text(DART_PARSER_SOURCE, encoding="utf-8")
            subprocess.run(
                [dart, "pub", "get"],
                cwd=tmp,
                check=True,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True,
            )
            result = subprocess.run(
                [dart, "run", "bin/main.dart", str(path)],
                cwd=tmp,
                check=True,
                capture_output=True,
                text=True,
            )
        data = json.loads(result.stdout or "{}")
        nodes: List[Dict[str, Any]] = data.get("nodes", [])
        comments = _extract_block_comments(source)
        for node in nodes:
            if not node.get("doc"):
                start_line = node.get("range", {}).get("start", {}).get("line")
                if start_line in comments:
                    node["doc"] = comments[start_line]
        return ParsedDart(nodes=nodes)

    def extract_nodes(self, module: ParsedDart) -> Iterable[Dict[str, Any]]:
        nodes: List[Dict[str, Any]] = []
        for sym in module.nodes:
            kind = sym.get("kind")
            typ = "block" if kind == "function" else ("class" if kind == "class" else "variable")
            nodes.append(
                {
                    "id": sym.get("name", ""),
                    "type": typ,
                    "display": sym.get("doc", ""),
                    "range": sym.get("range", {}),
                }
            )
        return nodes

    def extract_connections(self, module: ParsedDart) -> Iterable[Any]:
        return []
