from __future__ import annotations

from pathlib import Path

from visual_mode.parser.base import LanguageParser


class DummyParser(LanguageParser):
    """Minimal parser used for testing the base class."""

    def parse_file(self, path: str | Path) -> str:
        return Path(path).read_text()

    def extract_nodes(self, tree: str) -> list[str]:
        return tree.split()

    def extract_connections(self, tree: str) -> list[tuple[str, str]]:
        return []


def test_dummy_parser_roundtrip(tmp_path: Path) -> None:
    file = tmp_path / "sample.lang"
    file.write_text("alpha beta")
    parser = DummyParser()
    tree = parser.parse_file(file)
    nodes = list(parser.extract_nodes(tree))
    connections = list(parser.extract_connections(tree))
    assert nodes == ["alpha", "beta"]
    assert connections == []
