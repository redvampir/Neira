from __future__ import annotations

from textwrap import dedent
from pathlib import Path

from visual_mode.parser.objc_parser import ObjCParser


def test_objc_parser_extracts_comments(tmp_path: Path) -> None:
    header_code = dedent(
        '''
        // Sample class
        @interface Sample
        // Returns sum
        - (int)add:(int)a b:(int)b;
        /* Greets user */
        - (NSString *)greet;
        @end
        '''
    )

    impl_code = dedent(
        '''
        // Implementation
        @implementation Sample
        // Returns sum
        - (int)add:(int)a b:(int)b { return a + b; }
        /* Greets user */
        - (NSString *)greet { return @"hi"; }
        @end
        '''
    )

    header = tmp_path / "Sample.h"
    impl = tmp_path / "Sample.m"
    header.write_text(header_code)
    impl.write_text(impl_code)

    parser = ObjCParser()
    header_mod = parser.parse_file(header)
    impl_mod = parser.parse_file(impl)

    header_nodes = {node["id"]: node for node in parser.extract_nodes(header_mod)}
    impl_nodes = {node["id"]: node for node in parser.extract_nodes(impl_mod)}

    assert header_nodes["Sample"]["display"] == "Sample class"
    assert header_nodes["add:b:"]["display"] == "Returns sum"
    assert header_nodes["greet"]["display"] == "Greets user"
    assert impl_nodes["Sample"]["display"] == "Implementation"
    assert impl_nodes["add:b:"]["display"] == "Returns sum"
    assert impl_nodes["greet"]["display"] == "Greets user"
