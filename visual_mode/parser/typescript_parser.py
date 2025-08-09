from __future__ import annotations

"""TypeScript source parser for visual programming mode.

This parser invokes the TypeScript compiler API via ``node`` to analyse a
TypeScript source file.  It extracts top level function and class declarations
and resolves parameter and return types together with decorator information.
The extracted information mirrors that of other language parsers in this
package and can be consumed by the visual editor.
"""

from dataclasses import dataclass
from pathlib import Path
from typing import Any, Dict, Iterable, List
import json
import os
import shutil
import subprocess

from .base import LanguageParser


@dataclass
class ParsedTypeScript:
    """Container holding parsed information about a TypeScript module."""

    symbols: List[Dict[str, Any]]


class TypeScriptParser(LanguageParser):
    """Concrete :class:`LanguageParser` implementation for TypeScript."""

    def _ensure_node(self) -> str:
        node = shutil.which("node")
        if node is None:
            raise EnvironmentError("node executable not found")
        return node

    def parse_file(self, path: str | Path) -> ParsedTypeScript:
        path = Path(path)
        node = self._ensure_node()
        npm = shutil.which("npm")
        node_path = ""
        if npm is not None:
            try:  # pragma: no cover - best effort
                node_path = (
                    subprocess.run(
                        [npm, "root", "-g"], capture_output=True, text=True, check=True
                    ).stdout.strip()
                )
            except Exception:  # pragma: no cover - npm unavailable
                node_path = ""
        env = os.environ.copy()
        if node_path:
            env["NODE_PATH"] = node_path

        script = r"""
const ts = require('typescript');
const fs = require('fs');
const fileName = process.argv[1];
const sourceText = fs.readFileSync(fileName, 'utf8');
const options = { target: ts.ScriptTarget.Latest, module: ts.ModuleKind.CommonJS, experimentalDecorators: true };
const program = ts.createProgram([fileName], options);
const checker = program.getTypeChecker();
const source = program.getSourceFile(fileName);

function getDoc(node) {
  const tags = ts.getJSDocCommentsAndTags(node) || [];
  return tags.map(t => t.getFullText().trim()).join('\n');
}
function getDecorators(node) {
  if (ts.canHaveDecorators && ts.canHaveDecorators(node)) {
    const decs = ts.getDecorators ? ts.getDecorators(node) || [] : [];
    return decs.map(d => d.expression.getText());
  }
  return node.decorators ? node.decorators.map(d => d.expression.getText()) : [];
}
function toLoc(pos) {
  const lc = source.getLineAndCharacterOfPosition(pos);
  return { line: lc.line + 1, column: lc.character + 1 };
}
function paramType(p) {
  const type = checker.getTypeAtLocation(p);
  return checker.typeToString(type);
}
const nodes = [];
ts.forEachChild(source, node => {
  if (ts.isFunctionDeclaration(node) && node.name) {
    const sig = checker.getSignatureFromDeclaration(node);
    const returnType = sig ? checker.typeToString(checker.getReturnTypeOfSignature(sig)) : '';
    const params = node.parameters.map(p => ({ name: p.name.getText(), type: paramType(p) }));
    const decorators = getDecorators(node);
    nodes.push({
      kind: 'function',
      name: node.name.getText(),
      doc: getDoc(node),
      returnType,
      parameters: params,
      decorators,
      range: { start: toLoc(node.getStart()), end: toLoc(node.end) }
    });
  } else if (ts.isClassDeclaration(node) && node.name) {
    const decorators = getDecorators(node);
    nodes.push({
      kind: 'class',
      name: node.name.getText(),
      doc: getDoc(node),
      decorators,
      range: { start: toLoc(node.getStart()), end: toLoc(node.end) }
    });
  }
});
process.stdout.write(JSON.stringify({nodes}));
"""
        result = subprocess.run(
            [node, "-e", script, str(path)],
            check=True,
            capture_output=True,
            text=True,
            env=env,
        )
        data = json.loads(result.stdout)
        return ParsedTypeScript(symbols=data.get("nodes", []))

    def extract_nodes(self, module: ParsedTypeScript) -> Iterable[Dict[str, Any]]:
        nodes: List[Dict[str, Any]] = []
        for sym in module.symbols:
            node: Dict[str, Any] = {
                "id": sym.get("name", ""),
                "type": "block" if sym.get("kind") == "function" else "class",
                "display": sym.get("doc", ""),
                "range": sym.get("range", {}),
            }
            if sym.get("decorators"):
                node["decorators"] = sym["decorators"]
            if sym.get("kind") == "function":
                node["return_type"] = sym.get("returnType", "")
                node["parameters"] = sym.get("parameters", [])
            nodes.append(node)
        return nodes

    def extract_connections(self, module: ParsedTypeScript) -> Iterable[Any]:
        return []
