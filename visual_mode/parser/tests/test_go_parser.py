from __future__ import annotations

from pathlib import Path
from textwrap import dedent

from visual_mode.parser.go_parser import GoParser


def test_go_parser_comments(tmp_path: Path) -> None:
    code = dedent(
        """
        package main

        // Add adds two integers
        func Add(a int, b int) int {
            return a + b
        }

        // x variable
        var x int

        var (
            // y variable
            y int
            z int
        )
        """
    )
    file = tmp_path / "sample.go"
    file.write_text(code)

    parser = GoParser()
    module = parser.parse_file(file)
    nodes = list(parser.extract_nodes(module))
    mapping = {node["id"]: node for node in nodes}

    assert mapping["Add"]["display"] == "Add adds two integers"
    assert mapping["x"]["display"] == "x variable"
    assert mapping["y"]["display"] == "y variable"
    assert mapping["z"]["display"] == ""
    assert list(parser.extract_connections(module)) == []
