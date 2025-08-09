from __future__ import annotations

"""PHP source parser for visual programming mode.

This module leverages the Node.js :mod:`php-parser` package to analyse PHP
source files.  It extracts functions, classes and class properties together
with preceding or inline comments using ``//``, ``#`` and ``/* ... */``
styles.  The collected information mirrors the structure produced by other
language parsers and can be consumed by the visual editor.
"""

from dataclasses import dataclass
from pathlib import Path
from typing import Any, Dict, Iterable, List
import subprocess
import tempfile

from .base import LanguageParser

# ---------------------------------------------------------------------------
# Node.js helper management
# ---------------------------------------------------------------------------

_HELPER_SRC = r"""
const fs = require('fs');
const parser = require('php-parser');

const path = process.argv[2];
const code = fs.readFileSync(path, 'utf8');
const engine = new parser.Engine({
  parser: { extractDoc: true, php7: true },
  ast: { withPositions: true }
});
const ast = engine.parseCode(code, path);

function clean(text) {
  if (!text) return '';
  return text
    .replace(/^\s*(\/\/|#)/, '')
    .replace(/^\/\*/, '')
    .replace(/\*\/$/, '')
    .split('\n')
    .map(l => l.replace(/^\s*\*?/, '').trim())
    .join(' ')
    .trim()
    .replace(/\t/g, ' ');
}

function collectComments(node) {
  const out = [];
  const handle = (arr) => {
    if (!arr) return;
    for (const c of arr) out.push(clean(c.value));
  };
  handle(node.leadingComments);
  handle(node.trailingComments);
  if (node.body && node.body.trailingComments) handle(node.body.trailingComments);
  return out.join(' ').trim();
}

const nodes = [];
function addNode(name, kind, loc, doc) {
  const start = loc && loc.start ? loc.start : { line: 1, column: 0 };
  const end = loc && loc.end ? loc.end : start;
  nodes.push({ name, kind, start, end, doc });
}

function walk(node) {
  if (!node || typeof node !== 'object') return;
  switch (node.kind) {
    case 'function':
    case 'method':
    case 'class':
      addNode(node.name.name, node.kind, node.loc, collectComments(node));
      break;
    case 'propertystatement':
      const doc = collectComments(node);
      for (const prop of node.properties) {
        const pname = prop.name && prop.name.name ? prop.name.name : prop.name;
        addNode(pname, 'property', prop.loc || node.loc, doc);
      }
      break;
  }
  for (const key in node) {
    if (!Object.prototype.hasOwnProperty.call(node, key)) continue;
    const child = node[key];
    if (Array.isArray(child)) child.forEach(walk);
    else walk(child);
  }
}

walk(ast);
for (const n of nodes) {
  console.log(`${n.name}\t${n.kind}\t${n.start.line}\t${n.start.column + 1}\t${n.end.line}\t${n.end.column + 1}\t${n.doc}`);
}
"""

_TEMP_DIR = Path(tempfile.gettempdir()) / "visual_mode_php_parser"
_HELPER_JS = _TEMP_DIR / "_php_parser_helper.js"
_NODE_MODULES = _TEMP_DIR / "node_modules"
_PHP_PARSER = _NODE_MODULES / "php-parser"


def _ensure_helper() -> None:
    """Ensure the Node.js helper script and dependencies exist."""
    if not _TEMP_DIR.exists():
        _TEMP_DIR.mkdir(parents=True, exist_ok=True)
    if not _PHP_PARSER.exists():  # install npm package lazily
        subprocess.run(
            ['npm', 'init', '-y'],
            cwd=_TEMP_DIR,
            check=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
        subprocess.run(
            ['npm', 'install', 'php-parser@3'],
            cwd=_TEMP_DIR,
            check=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
    if not _HELPER_JS.exists() or _HELPER_JS.read_text() != _HELPER_SRC:
        _HELPER_JS.write_text(_HELPER_SRC)


# ---------------------------------------------------------------------------
# Parsed module container
# ---------------------------------------------------------------------------

@dataclass
class ParsedPHP:
    """Container holding parsed PHP declarations."""

    nodes: List[Dict[str, Any]]


# ---------------------------------------------------------------------------
# PHP parser implementation
# ---------------------------------------------------------------------------

class PHPParser(LanguageParser):
    """Concrete :class:`LanguageParser` implementation for PHP."""

    def parse_file(self, path: str | Path) -> ParsedPHP:
        _ensure_helper()
        result = subprocess.run(
            ['node', str(_HELPER_JS), str(path)],
            check=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
        )
        nodes: List[Dict[str, Any]] = []
        for line in result.stdout.strip().splitlines():
            parts = line.split('\t')
            if len(parts) < 6:
                continue
            name, kind, sl, sc, el, ec, *doc_parts = parts
            doc = '\t'.join(doc_parts).strip()
            nodes.append(
                {
                    'id': name,
                    'kind': kind,
                    'doc': doc,
                    'range': {
                        'start': {'line': int(sl), 'column': int(sc)},
                        'end': {'line': int(el), 'column': int(ec)},
                    },
                }
            )
        return ParsedPHP(nodes=nodes)

    def extract_nodes(self, module: ParsedPHP) -> Iterable[Dict[str, Any]]:
        parsed: List[Dict[str, Any]] = []
        for node in module.nodes:
            node_type = 'block' if node['kind'] in {'function', 'method', 'class'} else 'variable'
            parsed.append(
                {
                    'id': node['id'],
                    'type': node_type,
                    'display': node['doc'],
                    'range': node['range'],
                }
            )
        return parsed

    def extract_connections(self, module: ParsedPHP) -> Iterable[Any]:
        return []
