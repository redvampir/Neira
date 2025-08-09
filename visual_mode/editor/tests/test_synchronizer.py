from __future__ import annotations

from visual_mode.graph.builder import build_graph
from visual_mode.editor.synchronizer import Synchronizer


def _sample_metadata() -> dict:
    return {
        "blocks": [
            {
                "id": "b1",
                "range": {
                    "start": {"line": 1, "column": 1},
                    "end": {"line": 1, "column": 5},
                },
            }
        ],
        "variables": [
            {
                "id": "v1",
                "range": {
                    "start": {"line": 2, "column": 1},
                    "end": {"line": 2, "column": 3},
                },
            }
        ],
        "connections": [{"from": "b1", "to": "v1"}],
    }


def test_roundtrip_idempotent() -> None:
    metadata = _sample_metadata()
    initial_sync = Synchronizer("", metadata)
    original_text = initial_sync.to_text()

    graph = build_graph(metadata)
    sync = Synchronizer(original_text, metadata)
    sync.update_from_graph(graph)
    assert sync.to_text() == original_text


def test_updates_on_move_and_link_change() -> None:
    metadata = _sample_metadata()
    graph = build_graph(metadata)
    # Move node by updating its starting line
    graph["nodes"][0]["range"]["start"]["line"] = 10
    # Change connection target
    graph["links"][0]["to"] = "v2"

    sync = Synchronizer("", metadata)
    sync.update_from_graph(graph)

    assert sync.metadata["blocks"][0]["range"]["start"]["line"] == 10
    assert sync.metadata["connections"][0]["to"] == "v2"
