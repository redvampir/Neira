from __future__ import annotations

from textwrap import dedent
from pathlib import Path

from visual_mode.parser.scala_parser import ScalaParser


def test_class_and_method_doc_with_annotations(tmp_path: Path) -> None:
    code = dedent(
        """
        /** Main class doc */
        class Main {
          @deprecated
          // Multiply numbers
          def mul(a: Int, b: Int): Int = {
            a * b
          }
        }
        """
    )
    file = tmp_path / "Main.scala"
    file.write_text(code)

    parser = ScalaParser()
    module = parser.parse_file(file)
    nodes = list(parser.extract_nodes(module))
    mapping = {node["id"]: node for node in nodes}

    assert mapping["Main"]["display"] == "Main class doc"
    assert mapping["mul"]["display"] == "deprecated Multiply numbers"
    assert list(parser.extract_connections(module)) == []
