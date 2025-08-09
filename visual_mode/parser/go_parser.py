from __future__ import annotations

"""Go source parser for visual programming mode.

This parser delegates parsing to a small Go program that uses the standard
library's :mod:`go/ast` and :mod:`go/parser` packages.  The Go program extracts
all top-level function and variable declarations together with the preceding
comment groups ("doc comments").  The resulting information mirrors that of the
other language parsers in this package and can be consumed by the visual editor.
"""

from dataclasses import dataclass
import json
import subprocess
import tempfile
from pathlib import Path
from typing import Any, Dict, Iterable, List

from .base import LanguageParser

GO_PARSER_SOURCE = r"""
package main

import (
    "encoding/json"
    "go/ast"
    "go/parser"
    "go/token"
    "os"
    "strings"
)

type Position struct {
    Line   int `json:"line"`
    Column int `json:"column"`
}

type Range struct {
    Start Position `json:"start"`
    End   Position `json:"end"`
}

type Node struct {
    ID      string `json:"id"`
    Type    string `json:"type"`
    Display string `json:"display"`
    Range   Range  `json:"range"`
}

func pos(fset *token.FileSet, p token.Pos) Position {
    pos := fset.Position(p)
    return Position{Line: pos.Line, Column: pos.Column}
}

func main() {
    args := os.Args[1:]
    if len(args) > 0 && args[0] == "--" {
        args = args[1:]
    }
    if len(args) == 0 {
        return
    }
    filename := args[0]
    fset := token.NewFileSet()
    file, err := parser.ParseFile(fset, filename, nil, parser.ParseComments)
    if err != nil {
        panic(err)
    }
    nodes := []Node{}
    for _, decl := range file.Decls {
        switch d := decl.(type) {
        case *ast.FuncDecl:
            doc := ""
            if d.Doc != nil {
                doc = strings.TrimSpace(d.Doc.Text())
            }
            nodes = append(nodes, Node{
                ID:      d.Name.Name,
                Type:    "block",
                Display: doc,
                Range: Range{Start: pos(fset, d.Pos()), End: pos(fset, d.End())},
            })
        case *ast.GenDecl:
            if d.Tok == token.VAR {
                for _, spec := range d.Specs {
                    vs, ok := spec.(*ast.ValueSpec)
                    if !ok {
                        continue
                    }
                    doc := ""
                    if vs.Doc != nil {
                        doc = strings.TrimSpace(vs.Doc.Text())
                    } else if d.Doc != nil {
                        doc = strings.TrimSpace(d.Doc.Text())
                    }
                    for _, name := range vs.Names {
                        nodes = append(nodes, Node{
                            ID:      name.Name,
                            Type:    "variable",
                            Display: doc,
                            Range: Range{Start: pos(fset, name.Pos()), End: pos(fset, name.End())},
                        })
                    }
                }
            }
        }
    }
    enc := json.NewEncoder(os.Stdout)
    _ = enc.Encode(nodes)
}
"""


@dataclass
class ParsedGo:
    """Container holding parsed information about a Go module."""

    nodes: List[Dict[str, Any]]


class GoParser(LanguageParser):
    """Concrete :class:`LanguageParser` implementation for Go."""

    def parse_file(self, path: str | Path) -> ParsedGo:
        with tempfile.NamedTemporaryFile("w", suffix=".go", delete=False) as tmp:
            tmp.write(GO_PARSER_SOURCE)
            tmp_path = tmp.name
        try:
            proc = subprocess.run(
                ["go", "run", tmp_path, "--", str(path)],
                check=True,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True,
            )
        finally:
            Path(tmp_path).unlink(missing_ok=True)
        nodes: List[Dict[str, Any]] = json.loads(proc.stdout or "[]")
        return ParsedGo(nodes=nodes)

    def extract_nodes(self, module: ParsedGo) -> Iterable[Dict[str, Any]]:
        return module.nodes

    def extract_connections(self, module: ParsedGo) -> Iterable[Any]:
        return []
