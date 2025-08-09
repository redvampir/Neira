from __future__ import annotations

from textwrap import dedent
from pathlib import Path
import shutil
import pytest

if shutil.which("node") is None or shutil.which("npm") is None:  # pragma: no cover - environment
    pytest.skip("node and npm are required for php-parser", allow_module_level=True)

from visual_mode.parser.php_parser import PHPParser


def test_function_and_class_mapping(tmp_path: Path) -> None:
    code = dedent(
        """
        <?php
        // adds numbers
        function add($a, $b) {
            return $a + $b;
        }

        # Math utilities
        class Math {
            /* subtract numbers */
            public function sub($a, $b) {
                return $a - $b;
            }
            // the value property
            public $value = 0;
        }
        """
    )
    file = tmp_path / "sample.php"
    file.write_text(code)

    parser = PHPParser()
    module = parser.parse_file(file)
    nodes = {node["id"]: node for node in parser.extract_nodes(module)}

    assert nodes["add"]["display"] == "adds numbers"
    assert nodes["Math"]["display"] == "Math utilities"
    assert nodes["sub"]["display"] == "subtract numbers"
    assert nodes["value"]["display"] == "the value property"
    assert list(parser.extract_connections(module)) == []
