import sys
from pathlib import Path
import subprocess

sys.path.append(str(Path(__file__).resolve().parents[2]))

from code_editor.git_ui import GitUI, ConflictWindow


def _init_repo(path: Path) -> None:
    subprocess.run(["git", "init"], cwd=path, check=True)
    subprocess.run(["git", "config", "user.email", "test@example.com"], cwd=path, check=True)
    subprocess.run(["git", "config", "user.name", "Tester"], cwd=path, check=True)


def test_git_operations(tmp_path):
    repo = tmp_path / "repo"
    repo.mkdir()
    _init_repo(repo)

    remote = tmp_path / "remote.git"
    subprocess.run(["git", "init", "--bare", str(remote)], check=True)
    subprocess.run(["git", "remote", "add", "origin", str(remote)], cwd=repo, check=True)

    ui = GitUI(repo)

    (repo / "file.txt").write_text("hello\n")
    ui.commit("initial")
    assert ui.status_panel() == "\u2713 Clean"

    ui.branch("feature")
    branches = subprocess.check_output(["git", "branch"], cwd=repo, text=True)
    assert "feature" in branches

    (repo / "file.txt").write_text("hello world\n")
    status = ui.status_panel()
    assert "file.txt" in status and status.startswith("\u2717")

    ui.commit("update")
    ui.push("origin", "master")

    clone = tmp_path / "clone"
    subprocess.run(["git", "clone", str(remote), str(clone)], check=True)
    _init_repo(clone)  # set user config
    (clone / "file.txt").write_text("remote edit\n")
    subprocess.run(["git", "commit", "-am", "remote"], cwd=clone, check=True)
    subprocess.run(["git", "push", "origin", "master"], cwd=clone, check=True)

    ui.pull("origin", "master")
    assert "remote edit" in (repo / "file.txt").read_text()


def test_conflict_window_merge():
    win = ConflictWindow("a\nb\n", "a\nc\n")
    assert "c" in win.resolve("remote")
    assert "<table" in win.merged or "<<<<<<<" in win.merged
