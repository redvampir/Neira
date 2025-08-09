from __future__ import annotations

from textwrap import dedent
from pathlib import Path

from visual_mode.parser.javascript_parser import JavaScriptParser


def test_function_and_variable_mapping(tmp_path: Path) -> None:
    code = dedent(
        """
        // first variable
        const x = 1;
        const y = 2; // second variable
        /* third variable */
        let z = 3;
        /**
         * Adds two numbers
         */
        function add(a, b) {
            return a + b;
        }
        """
    )
    file = tmp_path / "sample.js"
    file.write_text(code)

    parser = JavaScriptParser()
    module = parser.parse_file(file)
    nodes = list(parser.extract_nodes(module))

    mapping = {node["id"]: node for node in nodes}

    assert mapping["x"]["display"] == "first variable"
    assert mapping["y"]["display"] == "second variable"
    assert mapping["z"]["display"] == "third variable"
    assert mapping["add"]["display"] == "Adds two numbers"
    assert list(parser.extract_connections(module)) == []
