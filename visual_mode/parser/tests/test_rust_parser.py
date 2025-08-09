from __future__ import annotations

from pathlib import Path
from textwrap import dedent

from visual_mode.parser.rust_parser import RustParser


def test_rust_parser_comments(tmp_path: Path) -> None:
    code = dedent(
        """
        /* Macro doc */
        macro_rules! greet {
            () => {};
        }

        /// Adds two numbers
        fn add(a: i32, b: i32) -> i32 {
            a + b
        }

        /* Subtracts b from a */
        fn sub(a: i32, b: i32) -> i32 {
            a - b
        }

        /// Utils module
        mod utils {
            /// triple the value
            pub fn triple(x: i32) -> i32 {
                x * 3
            }
        }
        """
    )
    file = tmp_path / "sample.rs"
    file.write_text(code)

    parser = RustParser()
    module = parser.parse_file(file)
    nodes = list(parser.extract_nodes(module))
    mapping = {node["id"]: node for node in nodes}

    assert mapping["greet"]["type"] == "macro"
    assert mapping["greet"]["display"] == "Macro doc"
    assert mapping["add"]["display"] == "Adds two numbers"
    assert mapping["sub"]["display"] == "Subtracts b from a"
    assert mapping["utils"]["type"] == "module"
    assert mapping["triple"]["display"] == "triple the value"
    assert list(parser.extract_connections(module)) == []
