from __future__ import annotations

"""Simple manager that loads and executes plugin hooks."""

from pathlib import Path
import importlib.util
import inspect
import logging
from typing import List

from .plugin_base import Plugin

logger = logging.getLogger(__name__)


class PluginManager:
    """Discover and manage plugins located in a directory."""

    def __init__(self, plugin_dir: str | Path = "plugins") -> None:
        self.plugin_dir = Path(plugin_dir)
        self.plugins: List[Plugin] = []
        self.load_plugins()

    # ------------------------------------------------------------------
    def load_plugins(self) -> None:
        """Import all plugin modules from ``self.plugin_dir``."""

        if not self.plugin_dir.exists():
            return

        for path in self.plugin_dir.glob("*.py"):
            if path.name.startswith("_"):
                continue
            spec = importlib.util.spec_from_file_location(path.stem, path)
            if spec and spec.loader:
                module = importlib.util.module_from_spec(spec)
                spec.loader.exec_module(module)  # type: ignore[call-arg]
                for _, obj in inspect.getmembers(module, inspect.isclass):
                    if issubclass(obj, Plugin) and obj is not Plugin:
                        try:
                            self.plugins.append(obj())
                        except Exception:
                            continue

    # ------------------------------------------------------------------
    def run_hook(self, hook: str, *args, **kwargs) -> None:
        for plugin in self.plugins:
            func = getattr(plugin, hook, None)
            if callable(func):
                try:
                    func(*args, **kwargs)
                except Exception as e:
                    logger.warning(
                        "Error in plugin %s for hook %s: %s",
                        plugin.__class__.__name__,
                        hook,
                        e,
                        exc_info=True,
                    )

    # Convenience wrappers ------------------------------------------------
    def on_draft(self, draft: str, context) -> None:
        self.run_hook("on_draft", draft, context)

    def on_gap_analysis(self, draft: str, gaps) -> None:
        self.run_hook("on_gap_analysis", draft, gaps)

    def on_finalize(self, response: str) -> None:
        self.run_hook("on_finalize", response)


__all__ = ["PluginManager", "Plugin"]
