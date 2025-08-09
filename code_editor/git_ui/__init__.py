from __future__ import annotations

"""Simple Git user-interface helpers for the code editor.

The module exposes :class:`GitUI` which wraps common git operations such as
commit, push, pull and branch creation.  It also exposes
:class:`ConflictWindow` that can render a very small visual merge of two text
variants.  These utilities are intentionally light-weight so they can be used
in unit tests without requiring a full graphical environment.
"""

from dataclasses import dataclass, field
from pathlib import Path
import subprocess
from typing import Optional

try:  # pragma: no cover - diff rendering is optional
    from difflib import HtmlDiff
except Exception:  # pragma: no cover - fall back to plain markers
    HtmlDiff = None  # type: ignore[assignment]


@dataclass
class GitUI:
    """Minimal facade around git command line operations."""

    repo_path: Path = Path.cwd()

    # ------------------------------------------------------------------
    def _run(self, *args: str) -> subprocess.CompletedProcess[str]:
        """Run a git command inside :attr:`repo_path`."""

        return subprocess.run(
            ["git", *args],
            cwd=self.repo_path,
            capture_output=True,
            text=True,
            check=True,
        )

    # ------------------------------------------------------------------
    def status(self) -> str:
        """Return the short git status for the repository."""

        result = self._run("status", "--short")
        return result.stdout.strip()

    # ------------------------------------------------------------------
    def status_panel(self) -> str:
        """Return a human friendly status string for a status bar."""

        short = self.status()
        return "\u2713 Clean" if not short else f"\u2717 {short}"

    # ------------------------------------------------------------------
    def commit(self, message: str) -> str:
        """Add all changes and create a commit with ``message``."""

        self._run("add", "-A")
        result = self._run("commit", "-m", message)
        return result.stdout.strip()

    # ------------------------------------------------------------------
    def push(self, remote: str = "origin", branch: str = "master") -> str:
        """Push the current branch to ``remote``."""

        result = self._run("push", remote, branch)
        return result.stdout.strip()

    # ------------------------------------------------------------------
    def pull(self, remote: str = "origin", branch: str = "master") -> str:
        """Pull updates from ``remote``."""

        result = self._run("pull", remote, branch)
        return result.stdout.strip()

    # ------------------------------------------------------------------
    def branch(self, name: str) -> str:
        """Create a new branch ``name``."""

        result = self._run("branch", name)
        return result.stdout.strip()


@dataclass
class ConflictWindow:
    """Represent a conflict resolution window with a visual merge."""

    local: str
    remote: str
    merged: str = field(init=False)

    def __post_init__(self) -> None:
        if HtmlDiff is None:
            self.merged = (
                "<<<<<<< LOCAL\n"
                f"{self.local}\n"
                "=======\n"
                f"{self.remote}\n"
                ">>>>>>> REMOTE"
            )
        else:
            diff = HtmlDiff()
            self.merged = diff.make_table(
                self.local.splitlines(),
                self.remote.splitlines(),
                fromdesc="local",
                todesc="remote",
                context=True,
                numlines=1,
            )

    # ------------------------------------------------------------------
    def resolve(self, side: str = "local") -> str:
        """Return the chosen version of the conflict."""

        if side == "remote":
            return self.remote
        if side == "merged":
            return self.merged
        return self.local
