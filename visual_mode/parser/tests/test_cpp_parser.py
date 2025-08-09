from __future__ import annotations

from textwrap import dedent
from pathlib import Path

from visual_mode.parser.cpp_parser import CppParser


def test_cpp_parser_handles_namespaces_and_templates(tmp_path: Path) -> None:
    code = dedent(
        '''
        namespace math {
        // Adds two numbers
        int add(int a, int b) { return a + b; }

        /* Multiplies two numbers */
        template <typename T>
        T mul(T a, T b) { return a * b; }
        }
        '''
    )

    src = tmp_path / "sample.cpp"
    src.write_text(code)

    parser = CppParser()
    module = parser.parse_file(src)
    nodes = {node["id"]: node for node in parser.extract_nodes(module)}

    assert nodes["math::add"]["display"] == "Adds two numbers"
    assert nodes["math::mul"]["display"] == "Multiplies two numbers"
