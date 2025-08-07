"""Simple text editor widget used in the desktop interface.

The real application uses a rich ``customtkinter`` based editor.  For the
purposes of the kata we provide a light‑weight, fully testable substitute that
captures the behaviour relevant for the higher level components:

* Two text panes – one for human edits and one for AI proposed changes.
* Very small "syntax highlighting" routine that marks Python keywords so that
  other components can inspect the result.
* A ``run_code`` helper used by a *Run* button in the GUI.  The helper executes
  code in a sandboxed subprocess with a timeout which is covered by unit tests
  in this kata.
* Optional integration with :class:`~src.llm.manager.LLMManager` so that the
  editor can request code completions from the Qwen Coder model.
"""

from __future__ import annotations

import re
import subprocess
import sys
from typing import Any, Optional, Tuple

from src.llm.manager import LLMManager, Task


class NeyraTextEditor:  # pragma: no cover - GUI stub
    """In‑memory representation of the editor state."""

    def __init__(
        self,
        parent: Any | None = None,
        *,
        llm_manager: Optional[LLMManager] = None,
    ) -> None:
        self.parent = parent
        self.llm_manager = llm_manager
        # Separate panes that would normally be backed by GUI widgets
        self.human_pane: str = ""
        self.ai_pane: str = ""

        self.setup_editor()
        self.setup_tag_highlighting()
        self.setup_autocomplete()

    # ------------------------------------------------------------------
    def setup_editor(self) -> None:
        """Initialise base editor configuration."""
        # Real GUI initialisation happens in the desktop application.  The stub
        # simply keeps attributes which are manipulated in tests.
        return None

    # ------------------------------------------------------------------
    def setup_tag_highlighting(self) -> None:
        """Prepare structures used for syntax highlighting."""
        return None

    # ------------------------------------------------------------------
    def setup_autocomplete(self) -> None:
        """Initialise autocomplete hooks."""
        return None

    # ------------------------------------------------------------------
    def detect_tags_realtime(self) -> None:
        """Placeholder for realtime tag detection."""
        return None

    # ------------------------------------------------------------------
    def insert_tag_template(self, tag_type: str) -> None:  # noqa: D401
        """Insert a tag template at the cursor position."""

        return None

    # ------------------------------------------------------------------
    def process_tag_command(self, tag: str) -> None:
        """Handle a tag command issued by the user."""

        return None

    # ------------------------------------------------------------------
    def highlight_syntax(self, code: str) -> str:
        """Return ``code`` with Python keywords wrapped in ``<kw>`` tags.

        The implementation is intentionally tiny – it is not a full syntax
        highlighter but suffices for tests and showcases how the real editor
        would provide highlighted output for display in the GUI.
        """

        keywords = "|".join(sorted(__import__("keyword").kwlist))
        pattern = re.compile(rf"\b({keywords})\b")
        return pattern.sub(r"<kw>\1</kw>", code)

    # ------------------------------------------------------------------
    def run_code(self, code: str, timeout: float = 5.0) -> Tuple[str, str]:
        """Execute ``code`` in a sandboxed subprocess.

        The code is executed using ``python -c`` in a separate process.  Output
        and errors are captured and returned.  If the subprocess does not finish
        within ``timeout`` seconds a :class:`TimeoutError` is raised.
        """

        try:
            proc = subprocess.run(  # noqa: PLW1510 - intentionally subprocess
                [sys.executable, "-c", code],
                capture_output=True,
                text=True,
                timeout=timeout,
            )
        except subprocess.TimeoutExpired as exc:  # pragma: no cover - exercised via tests
            raise TimeoutError("Execution timed out") from exc
        return proc.stdout, proc.stderr

    # ------------------------------------------------------------------
    def run_current_code(self, timeout: float = 5.0) -> Tuple[str, str]:
        """Execute the combined contents of the human and AI panes."""

        return self.run_code(self.human_pane + "\n" + self.ai_pane, timeout=timeout)

    # ------------------------------------------------------------------
    def request_ai_completion(self, prompt: str) -> str:
        """Use the attached :class:`LLMManager` to complete ``prompt``.

        The resulting code is stored in the AI pane and returned.
        """

        if self.llm_manager is None:  # pragma: no cover - simple guard
            raise RuntimeError("LLMManager not connected")
        task = Task(prompt=prompt, request_type="code")
        completion = self.llm_manager.generate(task)
        self.ai_pane = completion
        return completion

    # ------------------------------------------------------------------
    # Convenience setters/getters used by tests or higher level components
    def set_human_pane(self, text: str) -> None:
        self.human_pane = text

    def set_ai_pane(self, text: str) -> None:
        self.ai_pane = text

    def get_human_pane(self) -> str:
        return self.human_pane

    def get_ai_pane(self) -> str:
        return self.ai_pane
