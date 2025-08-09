from __future__ import annotations

from textwrap import dedent
from pathlib import Path

from visual_mode.parser.r_parser import RParser


def test_r_parser_comments(tmp_path: Path) -> None:
    code = dedent(
        """
        x <- 1  # first variable
        # variable y
        y <- 2

        # Adds two numbers
        add <- function(a, b) {
            a + b
        }
        """
    )
    file = tmp_path / "sample.R"
    file.write_text(code)

    parser = RParser()
    module = parser.parse_file(file)
    nodes = list(parser.extract_nodes(module))
    mapping = {node["id"]: node for node in nodes}

    assert mapping["x"]["display"] == "first variable"
    assert mapping["y"]["display"] == "variable y"
    assert mapping["add"]["display"] == "Adds two numbers"
    assert list(parser.extract_connections(module)) == []
