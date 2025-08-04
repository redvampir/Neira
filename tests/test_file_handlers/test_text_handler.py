
from src.file_handlers.text_handler import TextHandler


def test_read_and_save_txt(tmp_path):
    handler = TextHandler()
    original_text = "Привет, Нейра!"
    file_path = tmp_path / "sample.txt"
    file_path.write_text(original_text, encoding="utf-8")

    data = handler.read_file(str(file_path))
    assert data["content"] == original_text
    assert data["encoding"].lower() == "utf-8"

    new_file = tmp_path / "out.txt"
    assert handler.save_file(str(new_file), data)
    assert new_file.read_text(encoding=data["encoding"]) == original_text


def test_can_handle_extensions():
    handler = TextHandler()
    for ext in [".txt", ".md", ".rtf", ".tex"]:
        assert handler.can_handle(f"dummy{ext}")
