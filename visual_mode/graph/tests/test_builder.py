from __future__ import annotations

import json
from pathlib import Path

from visual_mode.graph.builder import build_graph, write_graph


def _sample_metadata() -> dict:
    return {
        "blocks": [
            {
                "id": "sum",
                "display": "Add",
                "i18n": {"es": "Suma"},
                "category": "math",
                "category_i18n": {"es": "matemáticas"},
            }
        ],
        "variables": [
            {"id": "x", "display": "X", "category": "math"}
        ],
        "connections": [
            {
                "from": "x",
                "to": "sum",
                "display": "value",
                "i18n": {"es": "valor"},
                "category": "data",
            }
        ],
    }


def test_build_graph_preserves_metadata() -> None:
    graph = build_graph(_sample_metadata())
    assert len(graph["nodes"]) == 2
    sum_node = next(node for node in graph["nodes"] if node["id"] == "sum")
    assert sum_node["display"] == "Add"
    assert sum_node["i18n"]["es"] == "Suma"
    assert sum_node["category"] == "math"
    assert sum_node["category_i18n"]["es"] == "matemáticas"
    link = graph["links"][0]
    assert link["from"] == "x"
    assert link["to"] == "sum"
    assert link["i18n"]["es"] == "valor"


def test_write_graph_creates_file(tmp_path: Path) -> None:
    out = write_graph(_sample_metadata(), tmp_path / "graph")
    assert out.suffix == ".neyra-graph"
    data = json.loads(out.read_text())
    assert "nodes" in data and "links" in data
    assert len(data["nodes"]) == 2
