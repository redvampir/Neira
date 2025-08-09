from pathlib import Path
from src.search.indexer import SearchIndexer
from src.search.ui import SearchPanel


def test_indexer_search_across_modes(tmp_path):
    paths = {}
    keywords = {}
    for mode in ["book", "code", "chat", "resources"]:
        mode_dir = tmp_path / mode
        mode_dir.mkdir()
        (mode_dir / "sample.txt").write_text(f"content for {mode}")
        paths[mode] = mode_dir
        keywords[mode] = mode
    indexer = SearchIndexer(paths)
    for mode, keyword in keywords.items():
        results = indexer.search(keyword)
        assert results and results[0].mode == mode


def test_indexer_updates_on_file_change(tmp_path):
    book_dir = tmp_path / "book"
    book_dir.mkdir()
    file = book_dir / "note.txt"
    file.write_text("initial")
    indexer = SearchIndexer({"book": book_dir})
    assert indexer.search("initial")
    # modify file
    file.write_text("updated content")
    indexer.update()
    assert not indexer.search("initial")
    assert indexer.search("updated")


def test_search_panel_filters_and_preview(tmp_path):
    book_dir = tmp_path / "book"
    book_dir.mkdir()
    (book_dir / "b.txt").write_text("book keyword")
    code_dir = tmp_path / "code"
    code_dir.mkdir()
    (code_dir / "c.txt").write_text("code keyword")
    indexer = SearchIndexer({"book": book_dir, "code": code_dir})

    panel = SearchPanel(indexer)
    panel.set_filters(["book"])
    results = panel.search("keyword")
    assert len(results) == 1
    assert results[0].mode == "book"
    assert "keyword" in results[0].preview
