from __future__ import annotations

"""Rust source parser for visual programming mode.

This parser delegates Rust syntax analysis to a small Rust program using the
``syn`` and ``proc_macro2`` crates. The program extracts top level functions,
modules and macro definitions together with their accompanying documentation
comments. Line ``///`` comments as well as block ``/* ... */`` comments are
considered for metadata. The resulting information mirrors that of the other
language parsers in this package and can be consumed by the visual editor.
"""

from dataclasses import dataclass
import json
import subprocess
import tempfile
from pathlib import Path
from typing import Any, Dict, Iterable, List

from .base import LanguageParser

RUST_PARSER_SOURCE = r"""
use std::collections::HashMap;
use std::env;
use std::fs;

use serde::Serialize;
use syn::{spanned::Spanned, Attribute, File, Item};

#[derive(Serialize)]
struct Position { line: usize, column: usize }

#[derive(Serialize)]
struct Range { start: Position, end: Position }

#[derive(Serialize)]
struct Node {
    id: String,
    #[serde(rename = "type")]
    typ: String,
    display: String,
    range: Range,
}

fn range(span: proc_macro2::Span) -> Range {
    let start = span.start();
    let end = span.end();
    Range {
        start: Position { line: start.line, column: start.column + 1 },
        end: Position { line: end.line, column: end.column + 1 },
    }
}

fn doc(attrs: &[Attribute]) -> String {
    let mut docs = Vec::new();
    for attr in attrs {
        if attr.path().is_ident("doc") {
            if let syn::Meta::NameValue(meta) = &attr.meta {
                if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(lit), .. }) = &meta.value {
                    docs.push(lit.value());
                }
            }
        }
    }
    docs.join("\n").trim().to_string()
}

fn extract_block_comments(src: &str) -> HashMap<usize, String> {
    let mut comments = HashMap::new();
    let lines: Vec<&str> = src.lines().collect();
    let mut pos = 0;
    while let Some(start) = src[pos..].find("/*") {
        let start_idx = pos + start;
        if let Some(end_rel) = src[start_idx + 2..].find("*/") {
            let end_idx = start_idx + 2 + end_rel;
            let body = &src[start_idx + 2..end_idx];
            let text = body
                .lines()
                .map(|l| l.trim().trim_start_matches('*').trim())
                .collect::<Vec<_>>()
                .join("\n")
                .trim()
                .to_string();
            let end_line = src[..end_idx + 2].chars().filter(|&c| c == '\n').count() + 1;
            let mut line = end_line + 1;
            while line <= lines.len() {
                let l = lines[line - 1].trim();
                if !l.is_empty() && !l.starts_with("//") && !l.starts_with("/*") {
                    comments.insert(line, text.clone());
                    break;
                }
                line += 1;
            }
            pos = end_idx + 2;
        } else {
            break;
        }
    }
    comments
}

fn handle_items(items: &[Item], comments: &HashMap<usize, String>, nodes: &mut Vec<Node>) {
    for item in items {
        match item {
            Item::Fn(f) => {
                let mut display = doc(&f.attrs);
                if display.is_empty() {
                    if let Some(c) = comments.get(&f.span().start().line) {
                        display = c.clone();
                    }
                }
                nodes.push(Node {
                    id: f.sig.ident.to_string(),
                    typ: "block".into(),
                    display,
                    range: range(f.span()),
                });
            }
            Item::Mod(m) => {
                let mut display = doc(&m.attrs);
                if display.is_empty() {
                    if let Some(c) = comments.get(&m.span().start().line) {
                        display = c.clone();
                    }
                }
                nodes.push(Node {
                    id: m.ident.to_string(),
                    typ: "module".into(),
                    display,
                    range: range(m.span()),
                });
                if let Some((_, items)) = &m.content {
                    handle_items(items, comments, nodes);
                }
            }
            Item::Macro(mac) => {
                let mut name = String::new();
                if let Some(ident) = &mac.ident {
                    name = ident.to_string();
                } else if let Some(seg) = mac.mac.path.segments.last() {
                    name = seg.ident.to_string();
                }
                let mut display = doc(&mac.attrs);
                if display.is_empty() {
                    if let Some(c) = comments.get(&mac.span().start().line) {
                        display = c.clone();
                    }
                }
                nodes.push(Node {
                    id: name,
                    typ: "macro".into(),
                    display,
                    range: range(mac.span()),
                });
            }
            _ => {}
        }
    }
}

fn main() {
    let path = env::args().nth(1).expect("missing path");
    let src = fs::read_to_string(&path).expect("read file");
    let file: File = syn::parse_file(&src).expect("parse file");
    let comments = extract_block_comments(&src);
    let mut nodes: Vec<Node> = Vec::new();
    handle_items(&file.items, &comments, &mut nodes);
    println!("{}", serde_json::to_string(&nodes).unwrap());
}
"""


@dataclass
class ParsedRust:
    """Container holding parsed information about a Rust crate."""

    nodes: List[Dict[str, Any]]


class RustParser(LanguageParser):
    """Concrete :class:`LanguageParser` implementation for Rust."""

    def parse_file(self, path: str | Path) -> ParsedRust:
        path = Path(path)
        with tempfile.TemporaryDirectory() as tmp:
            tmpdir = Path(tmp)
            (tmpdir / "src").mkdir()
            (tmpdir / "src" / "main.rs").write_text(RUST_PARSER_SOURCE, encoding="utf-8")
            cargo_toml = """
[package]
name = "rust_parser"
version = "0.1.0"
edition = "2021"

[dependencies]
syn = { version = "2", features = ["full"] }
proc-macro2 = { version = "1", features = ["span-locations"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
"""
            (tmpdir / "Cargo.toml").write_text(cargo_toml, encoding="utf-8")
            proc = subprocess.run(
                [
                    "cargo",
                    "run",
                    "--quiet",
                    "--manifest-path",
                    str(tmpdir / "Cargo.toml"),
                    "--",
                    str(path),
                ],
                check=True,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True,
            )
        nodes: List[Dict[str, Any]] = json.loads(proc.stdout or "[]")
        return ParsedRust(nodes=nodes)

    def extract_nodes(self, module: ParsedRust) -> Iterable[Dict[str, Any]]:
        return module.nodes

    def extract_connections(self, module: ParsedRust) -> Iterable[Any]:
        return []
