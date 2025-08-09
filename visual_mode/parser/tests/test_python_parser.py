from __future__ import annotations

from textwrap import dedent
from pathlib import Path

from visual_mode.parser.python_parser import PythonParser


def test_function_and_variable_mapping(tmp_path: Path) -> None:
    code = dedent(
        '''
        x = 1  # first variable
        y = 2
        """variable y"""

        def add(a, b):
            """Adds two numbers"""
            return a + b
        '''
    )
    file = tmp_path / "sample.py"
    file.write_text(code)

    parser = PythonParser()
    module = parser.parse_file(file)
    nodes = list(parser.extract_nodes(module))

    mapping = {node["id"]: node for node in nodes}

    assert mapping["add"]["display"] == "Adds two numbers"
    assert mapping["x"]["display"] == "first variable"
    assert mapping["y"]["display"] == "variable y"
    assert list(parser.extract_connections(module)) == []
