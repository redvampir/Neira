import json
import subprocess
import sys
from pathlib import Path

# Ensure project root is on sys.path for src layout
sys.path.append(str(Path(__file__).resolve().parents[1]))

from src.translation.manager import TranslationManager  # noqa: E402


def test_extract_identifiers():
    code = (
        "def add(x, y):\n"
        "    result = x + y\n"
        "    return result\n"
    )
    tm = TranslationManager()
    identifiers = tm.extract_identifiers(code)
    names = {(i.name, i.kind) for i in identifiers}
    assert ("add", "visual_block") in names
    assert ("x", "var") in names
    assert ("y", "var") in names
    assert ("result", "var") in names


def test_annotate_source_adds_and_updates():
    code = (
        "def add(x, y):\n"
        '    # @neyra:var id="result" display="Old"\n'
        "    result = x + y\n"
        "    return result\n"
    )
    tm = TranslationManager({"add": "Sum", "result": "Result"})
    annotated = tm.annotate_source(code)
    lines = annotated.splitlines()
    assert lines[0].startswith('# @neyra:visual_block id="add" display="Sum"')
    # Updated existing comment
    assert lines[2].strip() == '# @neyra:var id="result" display="Result"'


def test_generate_and_reverse_name():
    tm = TranslationManager()
    assert tm.generate_name("First number") == "first_number"
    assert tm.reverse_translate_name("first_number") == "First Number"


def test_annotate_project_script(tmp_path: Path):
    code = (
        "def add(x, y):\n"
        "    result = x + y\n"
        "    return result\n"
    )
    file = tmp_path / "module.py"
    file.write_text(code, encoding="utf-8")
    dictionary = {"add": "Sum", "result": "Result"}
    dict_path = tmp_path / "dict.json"
    dict_path.write_text(json.dumps(dictionary), encoding="utf-8")

    script = Path(__file__).resolve().parents[1] / "scripts" / "annotate_project.py"
    subprocess.check_call([sys.executable, str(script), str(tmp_path), "--dictionary", str(dict_path)])

    annotated = file.read_text(encoding="utf-8").splitlines()
    assert annotated[0].startswith('# @neyra:visual_block id="add" display="Sum"')
    assert annotated[2].strip() == '# @neyra:var id="result" display="Result"'
