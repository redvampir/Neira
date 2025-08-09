from __future__ import annotations

from textwrap import dedent
from pathlib import Path

from visual_mode.parser.kotlin_parser import KotlinParser


def test_kdoc_and_inline_comments(tmp_path: Path) -> None:
    code = dedent(
        """
        /** Adds two numbers */
        fun add(a: Int, b: Int): Int {
            return a + b
        }

        val answer = 42 // The ultimate answer
        """
    )
    file = tmp_path / "Main.kt"
    file.write_text(code)

    parser = KotlinParser()
    module = parser.parse_file(file)
    nodes = {n["id"]: n for n in parser.extract_nodes(module)}

    assert nodes["add"]["display"] == "Adds two numbers"
    assert nodes["answer"]["display"] == "The ultimate answer"
    assert list(parser.extract_connections(module)) == []
