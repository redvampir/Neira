import sys
from pathlib import Path

sys.path.append(str(Path(__file__).resolve().parents[2]))

from code_editor.lsp_client import LSPClient


def test_id_generation():
    client = LSPClient()
    first = client._next_id()
    second = client._next_id()
    assert second == first + 1
