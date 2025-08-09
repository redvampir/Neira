from pathlib import Path

from code_editor.lsp_client import LSPClient
from src.gui.text_editor import NeyraTextEditor
from web_preview import IframeView


def test_html_css_highlighting() -> None:
    editor = NeyraTextEditor()
    html = editor.highlight_syntax("<div>ok</div>")
    assert "<kw><div></kw>ok<kw></div></kw>" == html
    css = editor.highlight_syntax("body { color: red; }")
    assert "<kw>color</kw>: red" in css


def test_lsp_client_language_defaults() -> None:
    html_client = LSPClient(language="html")
    assert html_client.server_command[0].startswith("vscode-html")
    css_client = LSPClient(language="css")
    assert css_client.server_command[0].startswith("vscode-css")


def test_iframe_view_refresh(tmp_path: Path) -> None:
    html_file = tmp_path / "page.html"
    html_file.write_text("<p>hi</p>", encoding="utf-8")
    view = IframeView(html_file)

    # invalid trigger returns empty string
    assert view.open(trigger="wrong") == ""

    # correct trigger opens and shows content
    assert "hi" in view.open(trigger="open_preview")

    # saving new content should refresh automatically
    view.save("<p>bye</p>")
    assert "bye" in view.open(trigger="open_preview")
