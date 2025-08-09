from __future__ import annotations

from pathlib import Path
from textwrap import dedent

from visual_mode.parser.ruby_parser import RubyParser


def test_ruby_parser_comments(tmp_path: Path) -> None:
    code = dedent(
        """
        x = 1 # first variable

        =begin
        Adds two numbers
        =end
        def add(a, b)
          a + b
        end
        """
    )
    file = tmp_path / "sample.rb"
    file.write_text(code)

    parser = RubyParser()
    module = parser.parse_file(file)
    nodes = list(parser.extract_nodes(module))
    mapping = {node["id"]: node for node in nodes}

    assert mapping["x"]["display"] == "first variable"
    assert mapping["add"]["display"] == "Adds two numbers"
    assert list(parser.extract_connections(module)) == []
