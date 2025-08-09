from __future__ import annotations

from textwrap import dedent
from pathlib import Path

from visual_mode.parser.typescript_parser import TypeScriptParser


def test_typescript_functions_and_classes(tmp_path: Path) -> None:
    code = dedent(
        '''
        function add(a: number, b: number): number {
            return a + b;
        }

        @sealed
        class Greeter {
            constructor(private greeting: string) {}
        }
        '''
    )
    file = tmp_path / "sample.ts"
    file.write_text(code)

    parser = TypeScriptParser()
    module = parser.parse_file(file)
    nodes = list(parser.extract_nodes(module))

    mapping = {n["id"]: n for n in nodes}

    assert mapping["add"]["return_type"] == "number"
    assert mapping["add"]["parameters"] == [
        {"name": "a", "type": "number"},
        {"name": "b", "type": "number"},
    ]
    assert mapping["Greeter"].get("decorators") == ["sealed"]
    assert list(parser.extract_connections(module)) == []
