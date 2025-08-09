from pathlib import Path
import zipfile
import importlib.util
import sys

ROOT = Path(__file__).resolve().parents[2]
module_path = ROOT / "src/plugins/package_api.py"
spec = importlib.util.spec_from_file_location("package_api", module_path)
assert spec and spec.loader
package_api = importlib.util.module_from_spec(spec)
spec.loader.exec_module(package_api)  # type: ignore
upload_package = package_api.upload_package
download_package = package_api.download_package


def test_upload_and_download(tmp_path: Path) -> None:
    plugin_dir = tmp_path / "plugins"
    archive = tmp_path / "demo.zip"

    # create dummy zip file
    data_file = tmp_path / "data.txt"
    data_file.write_text("hi", encoding="utf-8")
    with zipfile.ZipFile(archive, "w") as zf:
        zf.write(data_file, "data.txt")

    stored_path = upload_package(archive, plugin_dir)
    assert stored_path.exists()

    content = download_package("demo", plugin_dir)
    assert content == archive.read_bytes()
