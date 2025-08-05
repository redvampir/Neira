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
    """Represent a single personality trait."""

    intensity: float = 0.0


@dataclass
class NeyraPersonalityCore:
    """Maintain a collection of personality traits.

    Each trait is stored as a key in the :pyattr:`traits` dictionary with a
    floating point intensity between 0.0 and 1.0.  The class exposes helper
    methods to modify these intensities and to obtain a qualitative reaction
    based on the current value.
    """

    traits: Dict[str, Trait] = field(default_factory=dict)

    def apply_trait(self, name: str, delta: float) -> float:
        """Modify ``name`` trait by ``delta`` and return the new intensity.

        The resulting intensity is clamped to the ``0.0``-``1.0`` range.
        ``delta`` can be positive or negative.  When the trait does not yet
        exist it is created with the provided delta.
        """

        trait = self.traits.setdefault(name, Trait())
        trait.intensity = max(0.0, min(1.0, trait.intensity + delta))
        return trait.intensity

    def get_trait(self, name: str) -> float:
        """Return the current intensity for ``name`` or ``0.0`` if missing."""

        trait = self.traits.get(name)
        return trait.intensity if trait else 0.0

    def get_reaction(self, name: str) -> str:
        """Return a qualitative reaction for the given trait ``name``.

        The reaction is a simple categorisation based on the current
        intensity of the trait: ``"weak"`` for values below ``0.3``,
        ``"moderate"`` for values between ``0.3`` and ``0.7`` and
        ``"strong"`` for higher intensities.
        """

        intensity = self.get_trait(name)
        if intensity >= 0.7:
            return "strong"
        if intensity >= 0.3:
            return "moderate"
        return "weak"

