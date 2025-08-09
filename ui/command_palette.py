from __future__ import annotations

"""Simple command palette for quickly executing registered actions.

The palette keeps a registry of commands that can be extended by core
modules or external plugins.  Commands are searchable both by their name
and the names of their parameters which allows quick discovery of
functionality.

The module exposes a :func:`register_command` function that plugins can
import and call during initialisation.  A :class:`CommandPalette`
instance provides a minimal interactive interface using ``prompt_toolkit``
and installs a global key binding (``Ctrl+Shift+P``) that activates the
palette.
"""

from dataclasses import dataclass
import inspect
from typing import Callable, Dict, Iterable, List, Tuple

try:  # pragma: no cover - optional dependency
    from prompt_toolkit import PromptSession
    from prompt_toolkit.completion import FuzzyWordCompleter
    from prompt_toolkit.key_binding import KeyBindings
except Exception:  # pragma: no cover - library may not be installed
    PromptSession = None  # type: ignore
    FuzzyWordCompleter = None  # type: ignore

    class _Binding:
        def __init__(self, keys: Tuple[str, ...], handler: Callable):
            self.keys = keys
            self.handler = handler

    class KeyBindings:  # type: ignore
        def __init__(self) -> None:
            self.bindings: List[_Binding] = []

        def add(self, *keys: str) -> Callable[[Callable], Callable]:
            def decorator(func: Callable) -> Callable:
                self.bindings.append(_Binding(keys, func))
                return func

            return decorator

    class PromptSession:  # type: ignore
        def prompt(self, *_args, **_kwargs) -> str:
            return ""

    class FuzzyWordCompleter:  # type: ignore
        def __init__(self, *_args, **_kwargs) -> None:  # pragma: no cover - minimal stub
            pass


CommandFunc = Callable[..., object]

# Global command registry -------------------------------------------------------
_registry: Dict[str, CommandFunc] = {}


def register_command(name: str, func: CommandFunc) -> None:
    """Register a callable under ``name``.

    Plugins can call this during import to expose new commands in the
    palette.
    """

    _registry[name] = func


# Command palette implementation ----------------------------------------------
@dataclass
class CommandPalette:
    """Command palette with fuzzy search across commands and arguments."""

    commands: Dict[str, CommandFunc] = None  # type: ignore[assignment]

    def __post_init__(self) -> None:
        self.commands = _registry
        self.session = PromptSession()
        self.key_bindings = KeyBindings()

        @self.key_bindings.add("c-S-p")
        def _activate(event) -> None:  # pragma: no cover - interactive
            self.activate()

    # ------------------------------------------------------------------
    def search(self, query: str) -> List[Tuple[str, CommandFunc]]:
        """Return commands matching ``query``.

        The search inspects both the command name and the names of the
        parameters accepted by the underlying callable.
        """

        q = query.lower()
        results: List[Tuple[str, CommandFunc]] = []
        for name, func in self.commands.items():
            params = inspect.signature(func).parameters
            if q in name.lower() or any(q in p.lower() for p in params):
                results.append((name, func))
        return results

    # ------------------------------------------------------------------
    def activate(self) -> None:  # pragma: no cover - interactive
        """Open an interactive palette allowing the user to execute commands."""

        if PromptSession is None or FuzzyWordCompleter is None:
            return
        completer = FuzzyWordCompleter(list(self.commands))
        cmd_name = self.session.prompt("Command: ", completer=completer)
        if not cmd_name:
            return
        cmd = self.commands.get(cmd_name)
        if not cmd:
            return
        sig = inspect.signature(cmd)
        kwargs = {}
        for param in sig.parameters.values():
            value = self.session.prompt(f"{param.name}: ")
            kwargs[param.name] = value
        cmd(**kwargs)  # type: ignore[arg-type]


__all__ = ["CommandPalette", "register_command"]
