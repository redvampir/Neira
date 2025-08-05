"""Core personality mechanics for Neyra.

This module stores personality traits and provides helper methods for
updating and querying them.  Traits are represented as floating point
intensities from ``0.0`` to ``1.0`` where higher values indicate a
stronger presence of the trait.
"""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import Dict


@dataclass
class Trait:
    """Represent a single personality trait.

    The ``intensity`` value lies between ``0.0`` and ``1.0`` and can be
    modified with :meth:`apply`.  A helper :meth:`reaction` method provides a
    qualitative categorisation of the current intensity.
    """

    intensity: float = 0.0

    def apply(self, delta: float) -> float:
        """Apply ``delta`` to ``intensity`` while keeping it in bounds."""

        self.intensity = max(0.0, min(1.0, self.intensity + delta))
        return self.intensity

    def reaction(self) -> str:
        """Return a qualitative reaction for the current intensity."""

        if self.intensity >= 0.7:
            return "strong"
        if self.intensity >= 0.3:
            return "moderate"
        return "weak"


@dataclass
class NeyraPersonalityCore:
    """Maintain a collection of personality traits.

    Each trait is stored as a :class:`Trait` instance inside the
    :pyattr:`traits` dictionary.  Helper methods allow modifying traits and
    obtaining qualitative reactions based on their current intensity.
    """

    traits: Dict[str, Trait] = field(default_factory=dict)

    def apply_trait(self, name: str, delta: float) -> float:
        """Modify ``name`` trait by ``delta`` and return the new intensity."""

        trait = self.traits.get(name)
        if trait is None:
            trait = self.traits.setdefault(name, Trait())
        return trait.apply(delta)

    def get_trait(self, name: str) -> float:
        """Return the current intensity for ``name`` or ``0.0`` if missing."""

        trait = self.traits.get(name)
        return trait.intensity if trait else 0.0

    def get_reaction(self, name: str) -> str:
        """Return a qualitative reaction for the given trait ``name``."""

        trait = self.traits.get(name)
        if trait is None:
            trait = Trait()
        return trait.reaction()

