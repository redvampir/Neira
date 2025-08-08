from __future__ import annotations

from dataclasses import dataclass, field
from typing import Dict, Iterable, List, Set


class BaseModule:
    """Base interface for components managed by ``LifecycleManager``."""

    def start(self) -> None:  # pragma: no cover - interface method
        """Initialise module resources."""

    def stop(self) -> None:  # pragma: no cover - interface method
        """Release any held resources."""

    def health_check(self) -> bool:
        """Return ``True`` when the module is healthy."""
        return True


@dataclass
class ModuleRegistration:
    module: BaseModule
    dependencies: List[str] = field(default_factory=list)


class LifecycleManager:
    """Coordinate start/stop order and health checks for modules."""

    def __init__(self) -> None:
        self._modules: Dict[str, ModuleRegistration] = {}
        self._start_order: List[str] = []

    # ------------------------------------------------------------------
    def register(
        self, name: str, module: BaseModule, dependencies: Iterable[str] | None = None
    ) -> None:
        """Register a module and its dependencies."""

        self._modules[name] = ModuleRegistration(
            module=module, dependencies=list(dependencies or [])
        )

    # ------------------------------------------------------------------
    def _resolve_order(self) -> List[str]:
        """Topologically sort modules to respect dependencies."""

        order: List[str] = []
        temp: Set[str] = set()
        perm: Set[str] = set()

        def visit(node: str) -> None:
            if node in perm:
                return
            if node in temp:
                raise ValueError(f"Circular dependency for module '{node}'")
            temp.add(node)
            for dep in self._modules[node].dependencies:
                if dep not in self._modules:
                    raise KeyError(f"Unknown dependency '{dep}' for module '{node}'")
                visit(dep)
            temp.remove(node)
            perm.add(node)
            order.append(node)

        for name in list(self._modules):
            visit(name)

        return order

    # ------------------------------------------------------------------
    def start_all(self) -> None:
        """Start all registered modules in dependency order."""

        self._start_order = self._resolve_order()
        for name in self._start_order:
            self._modules[name].module.start()

    # ------------------------------------------------------------------
    def stop_all(self) -> None:
        """Stop all modules in reverse start order."""

        for name in reversed(self._start_order):
            self._modules[name].module.stop()
        self._start_order = []

    # ------------------------------------------------------------------
    def check_health(self) -> None:
        """Run health checks for all started modules and restart unhealthy ones."""

        for name in list(self._start_order):
            module = self._modules[name].module
            healthy = False
            try:
                healthy = module.health_check()
            except Exception:
                healthy = False
            if not healthy:
                self.restart(name)

    # ------------------------------------------------------------------
    def restart(self, name: str) -> None:
        """Restart the specified module regardless of its health."""

        module = self._modules[name].module
        try:
            module.stop()
        finally:
            module.start()


__all__ = ["BaseModule", "LifecycleManager"]
