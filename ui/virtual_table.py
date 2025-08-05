"""Abstractions for a virtual tabletop."""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import Any, Dict


@dataclass
class VirtualTable:
    """Representation of a virtual table and its components."""

    components: Dict[str, Any] = field(default_factory=dict)

    def add_component(self, name: str, component: Any) -> None:
        """Register a component on the table."""
        self.components[name] = component

    def serialize(self) -> Dict[str, Any]:
        """Produce a serialisable mapping of the table components."""
        serialised: Dict[str, Any] = {}
        for name, component in self.components.items():
            if hasattr(component, "serialize"):
                serialised[name] = component.serialize()  # type: ignore[call-arg]
            elif isinstance(component, (dict, list, str, int, float, bool)) or component is None:
                serialised[name] = component
            else:
                serialised[name] = repr(component)
        return serialised

    def render(self) -> Dict[str, Any]:
        """Return data ready for front-end rendering."""
        return self.serialize()
