from __future__ import annotations

from pathlib import Path
from textwrap import dedent

from visual_mode.parser.swift_parser import SwiftParser


def test_swift_parser_comments(tmp_path: Path) -> None:
    code = dedent(
        """
        /// Adds two numbers
        func add(_ a: Int, _ b: Int) -> Int {
            return a + b
        }

        /* Subtracts b from a */
        func sub(_ a: Int, _ b: Int) -> Int {
            a - b
        }

        /// Utilities module
        struct Utils {
            /// triple the value
            func triple(_ x: Int) -> Int { x * 3 }
        }
        """
    )
    file = tmp_path / "sample.swift"
    file.write_text(code)

    parser = SwiftParser()
    module = parser.parse_file(file)
    nodes = list(parser.extract_nodes(module))
    mapping = {node["id"]: node for node in nodes}

    assert mapping["add"]["display"] == "Adds two numbers"
    assert mapping["sub"]["display"] == "Subtracts b from a"
    assert mapping["Utils"]["display"] == "Utilities module"
    assert mapping["triple"]["display"] == "triple the value"
    assert list(parser.extract_connections(module)) == []
