from __future__ import annotations

from textwrap import dedent
from pathlib import Path

from visual_mode.parser.matlab_parser import MatlabParser


def test_matlab_parser_comments(tmp_path: Path) -> None:
    code = dedent(
        """
        % variable a
        a = 1;

        %{
        block doc for b
        over lines
        %}
        b = 2;

        c = 3; % inline c

        % Adds two numbers
        function z = add(x, y)
            z = x + y;
        end
        """
    )
    file = tmp_path / "sample.m"
    file.write_text(code)

    parser = MatlabParser()
    module = parser.parse_file(file)
    nodes = list(parser.extract_nodes(module))
    mapping = {node["id"]: node for node in nodes}

    assert mapping["a"]["display"] == "variable a"
    assert mapping["b"]["display"] == "block doc for b over lines"
    assert mapping["c"]["display"] == "inline c"
    assert mapping["add"]["display"] == "Adds two numbers"
    assert list(parser.extract_connections(module)) == []
