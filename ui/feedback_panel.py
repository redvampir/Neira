from __future__ import annotations

"""Feedback form for collecting messages, logs and screenshots.

The panel provides a small interactive prompt where users can enter a
message and specify paths to log files or screenshots.  Submissions are
stored locally under ``userdata/feedback`` and can optionally be sent to a
remote server.
"""

from dataclasses import dataclass
from pathlib import Path
from typing import Iterable, List
from datetime import datetime
import shutil
import base64

from help.context_helper import helper
from ui import hotkey_manager

try:  # pragma: no cover - optional dependency
    from prompt_toolkit import PromptSession
    from prompt_toolkit.key_binding import KeyBindings
except Exception:  # pragma: no cover - library may not be installed
    class _Binding:
        def __init__(self, keys: tuple[str, ...], handler):
            self.keys = keys
            self.handler = handler

    class KeyBindings:  # type: ignore
        def __init__(self) -> None:
            self.bindings: List[_Binding] = []

        def add(self, *keys: str):
            def decorator(func):
                self.bindings.append(_Binding(keys, func))
                return func

            return decorator

    class PromptSession:  # type: ignore
        def prompt(self, *_args, **_kwargs) -> str:  # pragma: no cover - minimal stub
            return ""


ROOT_DIR = Path(__file__).resolve().parents[1]
FEEDBACK_DIR = ROOT_DIR / "userdata" / "feedback"


@dataclass
class FeedbackReport:
    """Container for user feedback data."""

    message: str
    logs: List[Path]
    screenshots: List[Path]


def _copy_files(paths: Iterable[Path], dest: Path) -> None:
    for p in paths:
        if p.exists():
            shutil.copy(p, dest / p.name)


def save_report(report: FeedbackReport) -> Path:
    """Persist ``report`` under :data:`FEEDBACK_DIR` and return its path."""

    FEEDBACK_DIR.mkdir(parents=True, exist_ok=True)
    ts = datetime.utcnow().strftime("%Y%m%d_%H%M%S")
    report_dir = FEEDBACK_DIR / ts
    report_dir.mkdir(parents=True, exist_ok=True)
    (report_dir / "message.txt").write_text(report.message, encoding="utf-8")
    _copy_files(report.logs, report_dir)
    _copy_files(report.screenshots, report_dir)
    return report_dir


def send_to_server(report: FeedbackReport, url: str) -> bool:
    """Send ``report`` to ``url`` via HTTP POST.  Returns success."""

    try:  # pragma: no cover - optional dependency
        import requests
    except Exception:  # pragma: no cover - requests may be absent
        return False

    payload = {
        "message": report.message,
        "logs": {
            p.name: base64.b64encode(p.read_bytes()).decode("ascii")
            for p in report.logs
            if p.exists()
        },
        "screenshots": {
            p.name: base64.b64encode(p.read_bytes()).decode("ascii")
            for p in report.screenshots
            if p.exists()
        },
    }

    try:  # pragma: no cover - network operation
        requests.post(url, json=payload, timeout=5)
        return True
    except Exception:
        return False


def submit_feedback(
    message: str,
    logs: Iterable[str | Path] = (),
    screenshots: Iterable[str | Path] = (),
    server_url: str | None = None,
) -> Path:
    """Create a :class:`FeedbackReport` and store it locally.

    If ``server_url`` is provided, the report is also sent to the remote
    server.  Returns the path to the stored report.
    """

    log_paths = [Path(p) for p in logs]
    screenshot_paths = [Path(p) for p in screenshots]
    report = FeedbackReport(message, log_paths, screenshot_paths)
    path = save_report(report)
    if server_url:
        send_to_server(report, server_url)
    return path


@dataclass
class FeedbackPanel:
    """Interactive feedback form accessible via ``Ctrl+F12``."""

    server_url: str | None = None

    def __post_init__(self) -> None:
        self.session = PromptSession()
        self.key_bindings = KeyBindings()

        @self.key_bindings.add("c-f12")
        def _activate(event) -> None:  # pragma: no cover - interactive
            self.activate()

        helper.install_f1(self.key_bindings, lambda: "feedback_panel")

    def activate(self) -> None:  # pragma: no cover - interactive
        if PromptSession is None:
            return
        message = self.session.prompt("Message: ")
        log_input = self.session.prompt("Log files (comma separated): ")
        shot_input = self.session.prompt("Screenshot paths (comma separated): ")
        logs = [p.strip() for p in log_input.split(",") if p.strip()]
        shots = [p.strip() for p in shot_input.split(",") if p.strip()]
        submit_feedback(message, logs, shots, self.server_url)


helper.register_hint(
    "feedback_panel",
    "Форма обратной связи (Ctrl+F12): отправьте сообщение, логи и скриншоты.",
)
hotkey_manager.register_hotkey("feedback_panel", "c-f12")


__all__ = ["FeedbackPanel", "submit_feedback"]
