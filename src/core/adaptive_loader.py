"""Adaptive component loader using :class:`ResourceManager`.

This module lazily imports Python components when needed and releases them
again when they are no longer required.  The :func:`enable` function attempts
to allocate resources for the requested component via
``ResourceManager.allocate`` before importing the module.  When
``disable`` is called, the module is removed from ``sys.modules`` and any
allocated resources are returned to the manager.

In addition the :func:`determine_active_components` helper can be used to
decide which components should remain active based on the current system load
as reported by :class:`ResourceManager`.
"""

from __future__ import annotations

import gc
import importlib
import sys
from pathlib import Path
from types import ModuleType
from typing import Dict, Iterable, List

import importlib.util


def _load_resource_manager() -> type:
    """Load ``ResourceManager`` without importing the full package."""

    path = Path(__file__).resolve().parent.parent / "iteration" / "resource_manager.py"
    spec = importlib.util.spec_from_file_location("_resource_manager", path)
    module = importlib.util.module_from_spec(spec)
    assert spec and spec.loader  # for type checkers
    sys.modules["_resource_manager"] = module
    spec.loader.exec_module(module)  # type: ignore[union-attr]
    return module.ResourceManager


ResourceManager = _load_resource_manager()

# Global resource manager used by the loader.
resource_manager = ResourceManager()

# Registry of currently loaded components and their resource allocations.
_loaded: Dict[str, ModuleType] = {}
_allocations: Dict[str, int] = {}


def enable(component: str, amount: int = 1) -> ModuleType:
    """Lazily import ``component`` and allocate resources for it.

    Parameters
    ----------
    component:
        Dotted module path of the component to load.
    amount:
        Amount of CPU units to request from :data:`resource_manager`.
    """

    if component in _loaded:
        return _loaded[component]

    if not resource_manager.allocate(component, amount):
        raise RuntimeError(f"insufficient resources for {component}")

    module = importlib.import_module(component)
    _loaded[component] = module
    _allocations[component] = amount
    return module


def disable(component: str) -> None:
    """Unload ``component`` and release its resources."""

    module = _loaded.pop(component, None)
    if module is None:
        return

    resource_manager.release(component)
    sys.modules.pop(component, None)
    _allocations.pop(component, None)

    # Trigger garbage collection to free memory eagerly.
    gc.collect()


def determine_active_components(
    load_profile: str, candidates: Iterable[str]
) -> List[str]:
    """Return the subset of ``candidates`` that should stay active.

    The decision is based on the current CPU and memory usage reported by
    :data:`resource_manager` in combination with ``load_profile``.  The
    ``load_profile`` can be one of ``"low"``, ``"medium"`` or ``"high"`` and
    controls how aggressively components are pruned when system resources are
    under pressure.
    """

    cpu_usage, mem_usage = resource_manager.update_usage()
    thresholds = {"low": 90.0, "medium": 75.0, "high": 50.0}
    limit = thresholds.get(load_profile, 75.0)

    if cpu_usage > limit or mem_usage > limit:
        return []
    return list(candidates)


__all__ = [
    "enable",
    "disable",
    "determine_active_components",
    "resource_manager",
]

