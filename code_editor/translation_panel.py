"""Utilities for translating and annotating code segments.

The :class:`TranslationPanel` is a lightweight, non-GUI helper that mimics
behaviour of an IDE translation widget.  It can highlight code lines that miss
comments, provide very naive translation suggestions and insert template
annotations.

The goal of this module is not to implement a full translation workflow but to
offer a small, easily testable façade used in unit tests.  The panel loads its
configuration from ``config/editor.yaml`` where the ``auto_update`` flag controls
whether bulk annotation updates should be performed automatically.
"""

from __future__ import annotations

from dataclasses import dataclass, field
from pathlib import Path
from typing import Dict, List

from src.translation.profiles import (
    get_active_profile,
    set_active_profile,
)

try:  # pragma: no cover - optional dependency
    import yaml
except Exception:  # pragma: no cover - fallback when PyYAML is missing
    yaml = None  # type: ignore[assignment]

# ---------------------------------------------------------------------------
# Configuration loading
CONFIG_PATH = Path(__file__).resolve().parents[1] / "config" / "editor.yaml"


def _load_config() -> Dict[str, bool]:
    """Load editor configuration from :data:`CONFIG_PATH`.

    The function returns an empty dictionary if the configuration file is
    missing or unreadable.  Only the small subset needed by the translation
    panel is parsed.
    """

    if CONFIG_PATH.exists():
        if yaml is not None:
            with CONFIG_PATH.open("r", encoding="utf-8") as fh:
                data = yaml.safe_load(fh) or {}
                if isinstance(data, dict):
                    return {str(k): bool(v) for k, v in data.items()}
        else:  # Fallback simplistic parser when PyYAML is unavailable
            text = CONFIG_PATH.read_text(encoding="utf-8")
            return {"auto_update": "auto_update: true" in text.lower()}
    return {}


# ---------------------------------------------------------------------------
@dataclass
class TranslationPanel:
    """Provide helpers for translation and code annotation.

    Parameters
    ----------
    templates:
        Predefined ``@neyra:`` templates to suggest to the user.
    auto_update:
        When ``True`` the :meth:`bulk_update_annotations` method will insert
        missing comments automatically.  The default is read from
        ``config/editor.yaml``.
    """

    templates: List[str] = field(
        default_factory=lambda: ["@neyra:todo", "@neyra:fix", "@neyra:note"]
    )
    auto_update: bool = field(
        default_factory=lambda: _load_config().get("auto_update", False)
    )
    dictionary: Dict[str, str] = field(
        default_factory=lambda: dict(get_active_profile().dictionary)
    )
    profile_name: str = field(
        default_factory=lambda: get_active_profile().name
    )

    # ------------------------------------------------------------------
    def highlight_uncommented(self, text: str) -> List[int]:
        """Return line numbers of code that lack comments.

        The function performs a very small static analysis: any non-empty line
        that does not contain ``#`` or ``//`` is considered uncommented and its
        1-based line number is returned.  Empty lines are ignored.
        """

        lines = text.splitlines()
        uncommented: List[int] = []
        for idx, line in enumerate(lines, start=1):
            stripped = line.strip()
            if not stripped:
                continue
            if "#" in stripped or "//" in stripped:
                continue
            uncommented.append(idx)
        return uncommented

    # ------------------------------------------------------------------
    def suggest(self, text: str, trigger: str | None = None) -> Dict[str, List[str] | str]:
        """Return translation and template suggestions.

        Parameters
        ----------
        text:
            The text segment to translate.
        trigger:
            Hotkey used to invoke the suggestion.  Only ``"ctrl_enter"``
            triggers suggestions; otherwise an empty dictionary is returned.
        """

        if trigger != "ctrl_enter":
            return {}

        translation = self.dictionary.get(text)
        if translation is None:
            # The fallback "translation" is intentionally naive – it simply
            # reverses the string.  The goal is to have deterministic behaviour
            # for tests without pulling in heavy NLP dependencies.
            translation = text[::-1]
        return {"translation": translation, "templates": list(self.templates)}

    # ------------------------------------------------------------------
    def bulk_update_annotations(self, files: Dict[str, str]) -> Dict[str, str]:
        """Insert placeholder comments into uncommented code blocks.

        Parameters
        ----------
        files:
            Mapping of ``name -> code`` snippets.

        Returns
        -------
        dict
            Updated mapping with ``# TODO`` comments appended to uncommented
            lines.  If :attr:`auto_update` is ``False`` the input is returned
            unchanged.
        """

        if not self.auto_update:
            return files

        updated: Dict[str, str] = {}
        for name, content in files.items():
            new_lines: List[str] = []
            for idx, line in enumerate(content.splitlines(), start=1):
                stripped = line.strip()
                if stripped and "#" not in stripped and "//" not in stripped:
                    new_lines.append(f"{line}  # TODO")
                else:
                    new_lines.append(line)
            updated[name] = "\n".join(new_lines)
        return updated

    # ------------------------------------------------------------------ Profile management
    def select_profile(self, name: str) -> None:
        """Switch to another translation profile."""

        set_active_profile(name)
        profile = get_active_profile()
        self.dictionary = dict(profile.dictionary)
        self.profile_name = profile.name


# ---------------------------------------------------------------------------
def localization_menu_action(files: Dict[str, str]) -> Dict[str, str]:
    """Entry point used by the ``Tools → Localization`` menu.

    This thin wrapper simply instantiates :class:`TranslationPanel` and delegates
    to :meth:`bulk_update_annotations`.  It exists so that the menu system can
    call a predictable function without caring about class instantiation.
    """

    panel = TranslationPanel()
    return panel.bulk_update_annotations(files)
