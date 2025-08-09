from __future__ import annotations

from textwrap import dedent
from pathlib import Path

from visual_mode.parser.c_parser import CParser


def test_c_parser_handles_macros_and_headers(tmp_path: Path) -> None:
    header_code = dedent(
        '''
        /* Macro for constant */
        #define FOO 42

        /* Adds two numbers */
        int add(int a, int b);
        '''
    )

    source_code = dedent(
        '''
        #include "sample.h"
        /* Implementation of add */
        int add(int a, int b) {
            return a + b;
        }
        '''
    )

    header = tmp_path / "sample.h"
    source = tmp_path / "sample.c"
    header.write_text(header_code)
    source.write_text(source_code)

    parser = CParser()
    header_mod = parser.parse_file(header)
    source_mod = parser.parse_file(source)

    header_nodes = {node["id"]: node for node in parser.extract_nodes(header_mod)}
    source_nodes = {node["id"]: node for node in parser.extract_nodes(source_mod)}

    assert header_nodes["FOO"]["display"] == "Macro for constant"
    assert header_nodes["add"]["display"] == "Adds two numbers"
    assert source_nodes["add"]["display"] == "Implementation of add"
