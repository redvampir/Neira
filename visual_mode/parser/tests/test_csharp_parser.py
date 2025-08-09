from __future__ import annotations

from textwrap import dedent
from pathlib import Path
import pytest

# Skip if pythonnet or Roslyn assemblies are not available
clr = pytest.importorskip("clr")
try:  # pragma: no cover - ensure assemblies
    clr.AddReference("Microsoft.CodeAnalysis")
    clr.AddReference("Microsoft.CodeAnalysis.CSharp")
except Exception:  # pragma: no cover - skip when assemblies missing
    pytest.skip("Roslyn assemblies not available", allow_module_level=True)

from visual_mode.parser.csharp_parser import CSharpParser


def test_method_and_field_mapping(tmp_path: Path) -> None:
    code = dedent(
        """
        /// <summary>Calculator</summary>
        public class Calc {
            /// <summary>Add numbers</summary>
            public int Add(int a, int b) { return a + b; }

            public int value = 0; // initial value
        }
        """
    )
    file = tmp_path / "Calc.cs"
    file.write_text(code)

    parser = CSharpParser()
    module = parser.parse_file(file)
    nodes = {node["id"]: node for node in parser.extract_nodes(module)}

    assert nodes["Add"]["display"] == "Add numbers"
    assert nodes["value"]["display"] == "initial value"
    assert list(parser.extract_connections(module)) == []
