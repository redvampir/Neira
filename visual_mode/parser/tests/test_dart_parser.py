from __future__ import annotations

import shutil
from pathlib import Path
from textwrap import dedent

import pytest

if shutil.which("dart") is None:  # pragma: no cover - environment specific
    pytest.skip("dart executable not available", allow_module_level=True)

from visual_mode.parser.dart_parser import DartParser


def test_dart_parser_comments(tmp_path: Path) -> None:
    code = dedent(
        '''
        /* first variable */
        int x = 1;

        /// Adds two numbers
        int add(int a, int b) {
          return a + b;
        }

        /* Greeting class */
        class Greeter {}
        '''
    )
    file = tmp_path / "sample.dart"
    file.write_text(code)

    parser = DartParser()
    module = parser.parse_file(file)
    nodes = list(parser.extract_nodes(module))

    mapping = {node["id"]: node for node in nodes}

    assert mapping["x"]["display"] == "first variable"
    assert mapping["add"]["display"] == "Adds two numbers"
    assert mapping["Greeter"]["display"] == "Greeting class"
    assert list(parser.extract_connections(module)) == []
