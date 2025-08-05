"""Campaign interface configuration and serialization."""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import Any, Dict


@dataclass
class CampaignInterface:
    """Container for campaign UI sections.

    Parameters
    ----------
    sections:
        Mapping of section name to the object representing that section.
    """

    sections: Dict[str, Any] = field(default_factory=dict)

    def add_section(self, name: str, section: Any) -> None:
        """Register a new section under ``name``."""
        self.sections[name] = section

    # serialization helpers -------------------------------------------------
    def serialize(self) -> Dict[str, Any]:
        """Return a JSON-serialisable representation of the sections.

        ``section`` values that expose their own ``serialize`` method will be
        delegated to.  Otherwise the value is used as-is, which means basic
        Python data structures are passed straight through while complex
        objects fall back to ``repr``.
        """

        serialised: Dict[str, Any] = {}
        for name, section in self.sections.items():
            if hasattr(section, "serialize"):
                serialised[name] = section.serialize()  # type: ignore[call-arg]
            elif isinstance(section, (dict, list, str, int, float, bool)) or section is None:
                serialised[name] = section
            else:
                serialised[name] = repr(section)
        return serialised

    def render(self) -> Dict[str, Any]:
        """Expose the interface in a form suitable for front-end rendering."""
        return self.serialize()
