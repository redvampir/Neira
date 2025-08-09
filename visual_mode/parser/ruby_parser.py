from __future__ import annotations

"""Ruby source parser for visual programming mode.

This parser delegates Ruby syntax analysis to a small helper script using
Ruby's :mod:`ripper` library. The helper extracts top level method
definitions and variable assignments together with inline ``#`` comments or
preceding ``=begin/=end`` documentation blocks. The collected information
mirrors the structure produced by other language parsers and can be consumed by
the visual editor.
"""

from dataclasses import dataclass
import json
import subprocess
import tempfile
from pathlib import Path
from typing import Any, Dict, Iterable, List

from .base import LanguageParser

_HELPER_SRC = r"""
require 'ripper'
require 'json'

path = ARGV[0]
src = File.read(path)

tokens = Ripper.lex(src)
lines = src.lines
comments = {}
line_has_code = Hash.new(false)
block_comments = []
pending_block = nil

# Collect inline and block comments
for pos, type, token, state in tokens
  line = pos[0]
  case type
  when :on_comment
    txt = token.sub(/^#\s?/, '').strip
    if line_has_code[line]
      comments[line] = txt
    end
  when :on_embdoc_beg
    pending_block = { text: '' }
  when :on_embdoc
    pending_block[:text] << token
  when :on_embdoc_end
    text = pending_block ? pending_block[:text] : ''
    text = text.lines.map { |l| l.strip }.join(" ").strip
    block_comments << [line, text]
    pending_block = nil
  else
    unless [:on_sp, :on_ignored_sp, :on_nl, :on_ignored_nl, :on_comment, :on_embdoc, :on_embdoc_beg, :on_embdoc_end].include?(type)
      line_has_code[line] = true
    end
  end
end

# Associate block comments with the first following code line
block_comments.each do |end_line, text|
  i = end_line
  while i < lines.length
    i += 1
    l = lines[i - 1]
    next if l.strip.empty? || l.strip.start_with?('#')
    comments[i] = text
    break
  end
end

sexp = Ripper.sexp_raw(src)

def find_max_line(node)
  return 0 unless node.is_a?(Array)
  max = 0
  node.each do |child|
    if child.is_a?(Array)
      if child[0].is_a?(Symbol) && child[0].to_s.start_with?('@')
        max = [max, child[2][0]].max
      else
        max = [max, find_max_line(child)].max
      end
    end
  end
  max
end

def walk(node, depth, out, comments)
  return unless node.is_a?(Array)
  type = node[0]
  if type == :def && depth == 0
    ident = node[1]
    name = ident[1]
    line, col = ident[2]
    doc = comments[line] || ""
    out << {
      id: name,
      kind: 'function',
      doc: doc,
      start_line: line,
      start_col: col + 1,
      end_line: find_max_line(node),
      end_col: 1
    }
  elsif type == :assign && depth == 0
    var = node[1]
    if var[0] == :var_field && var[1][0] == :@ident
      name = var[1][1]
      line, col = var[1][2]
      doc = comments[line] || ""
      out << {
        id: name,
        kind: 'variable',
        doc: doc,
        start_line: line,
        start_col: col + 1,
        end_line: line,
        end_col: col + 1
      }
    end
  end
  node.each do |child|
    next unless child.is_a?(Array)
    new_depth = depth
    new_depth += 1 if [:def, :class, :module].include?(type)
    walk(child, new_depth, out, comments)
  end
end

nodes = []
walk(sexp, 0, nodes, comments)
puts JSON.dump(nodes)
"""

_TEMP_DIR = Path(tempfile.gettempdir()) / "visual_mode_ruby_parser"
_HELPER_RB = _TEMP_DIR / "_ruby_parser_helper.rb"

def _ensure_helper() -> None:
    """Ensure the Ruby helper script exists on disk."""
    if not _TEMP_DIR.exists():
        _TEMP_DIR.mkdir(parents=True, exist_ok=True)
    if not _HELPER_RB.exists() or _HELPER_RB.read_text(encoding="utf-8") != _HELPER_SRC:
        _HELPER_RB.write_text(_HELPER_SRC, encoding="utf-8")


@dataclass
class ParsedRuby:
    """Container holding parsed Ruby declarations."""

    nodes: List[Dict[str, Any]]


class RubyParser(LanguageParser):
    """Concrete :class:`LanguageParser` implementation for Ruby."""

    def parse_file(self, path: str | Path) -> ParsedRuby:
        _ensure_helper()
        result = subprocess.run(
            ["ruby", str(_HELPER_RB), str(path)],
            check=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
        )
        nodes: List[Dict[str, Any]] = json.loads(result.stdout or "[]")
        return ParsedRuby(nodes=nodes)

    def extract_nodes(self, module: ParsedRuby) -> Iterable[Dict[str, Any]]:
        parsed: List[Dict[str, Any]] = []
        for node in module.nodes:
            parsed.append(
                {
                    "id": node["id"],
                    "type": "block" if node["kind"] == "function" else "variable",
                    "display": node["doc"],
                    "range": {
                        "start": {"line": node["start_line"], "column": node["start_col"]},
                        "end": {"line": node["end_line"], "column": node["end_col"]},
                    },
                }
            )
        return parsed

    def extract_connections(self, module: ParsedRuby) -> Iterable[Any]:
        return []
