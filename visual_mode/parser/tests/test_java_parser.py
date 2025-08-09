from __future__ import annotations

from textwrap import dedent
from pathlib import Path

from visual_mode.parser.java_parser import JavaParser


def test_package_class_method_mapping(tmp_path: Path) -> None:
    code = dedent(
        """
        /* Package comment */
        package example;

        /**
         * Main class doc
         */
        public class Main {
            /* Multiply numbers */
            public int mul(int a, int b) {
                return a * b;
            }
        }
        """
    )
    file = tmp_path / "Main.java"
    file.write_text(code)

    parser = JavaParser()
    module = parser.parse_file(file)
    nodes = list(parser.extract_nodes(module))
    mapping = {node["id"]: node for node in nodes}

    assert mapping["example"]["display"] == "Package comment"
    assert mapping["Main"]["display"] == "Main class doc"
    assert mapping["mul"]["display"] == "Multiply numbers"
    assert list(parser.extract_connections(module)) == []
