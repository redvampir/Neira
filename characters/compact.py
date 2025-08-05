"""Compact character model with limited information storage."""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import Any, Dict, List
import json


@dataclass
class CompactCharacter:
    """Representation of a character with compressed trait data.

    Only a limited number of ``core_traits`` and ``story_moments`` are stored
    to keep the object lightweight. The class provides helpers for expanding
    the compact data using templates and for extracting new traits from a
    textual context while respecting the size limits.
    """

    name: str
    core_traits: List[str] = field(default_factory=list)
    story_moments: List[str] = field(default_factory=list)

    #: Maximum number of stored core traits.
    MAX_CORE_TRAITS: int = 5
    #: Maximum number of stored story moments.
    MAX_STORY_MOMENTS: int = 10

    def _trim_lists(self) -> None:
        """Ensure internal lists do not exceed the configured limits."""
        if len(self.core_traits) > self.MAX_CORE_TRAITS:
            self.core_traits[:] = self.core_traits[-self.MAX_CORE_TRAITS :]
        if len(self.story_moments) > self.MAX_STORY_MOMENTS:
            self.story_moments[:] = self.story_moments[-self.MAX_STORY_MOMENTS :]

    def _expand_from_templates(
        self,
        trait_templates: Dict[str, str],
        moment_templates: Dict[str, str],
    ) -> Dict[str, List[str]]:
        """Expand compact data using template mappings.

        Parameters
        ----------
        trait_templates:
            Mapping of trait identifiers to their detailed descriptions.
        moment_templates:
            Mapping of moment identifiers to their detailed descriptions.

        Returns
        -------
        Dict[str, List[str]]
            Expanded ``core_traits`` and ``story_moments`` lists.
        """

        expanded_traits = [
            trait_templates.get(trait, trait) for trait in self.core_traits
        ]
        expanded_moments = [
            moment_templates.get(moment, moment) for moment in self.story_moments
        ]
        return {
            "core_traits": expanded_traits,
            "story_moments": expanded_moments,
        }

    def _extract_new_traits(
        self, context: str, trait_patterns: Dict[str, str]
    ) -> List[str]:
        """Extract new traits from a context string.

        The ``trait_patterns`` mapping relates trait names to simple substrings
        that, if present in ``context``, indicate that the trait should be
        attached to the character. Newly discovered traits are appended and
        the lists are trimmed to respect the maximum sizes.

        Parameters
        ----------
        context:
            Textual context to search for traits.
        trait_patterns:
            Mapping of trait names to search substrings.

        Returns
        -------
        List[str]
            The updated list of ``core_traits``.
        """

        for trait, pattern in trait_patterns.items():
            if pattern in context and trait not in self.core_traits:
                self.core_traits.append(trait)
                self._trim_lists()
        return self.core_traits

    # ------------------------------------------------------------------
    # Serialization helpers
    # ------------------------------------------------------------------
    def to_dict(self) -> Dict[str, Any]:
        """Serialize the character to a JSON-serializable dict."""
        return {
            "name": self.name,
            "core_traits": list(self.core_traits),
            "story_moments": list(self.story_moments),
        }

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "CompactCharacter":
        """Deserialize a :class:`CompactCharacter` from a dictionary."""
        return cls(
            name=data.get("name", ""),
            core_traits=list(data.get("core_traits", [])),
            story_moments=list(data.get("story_moments", [])),
        )

    def to_json(self) -> str:
        """Serialize the character to a JSON string."""
        return json.dumps(self.to_dict())

    @classmethod
    def from_json(cls, data: str) -> "CompactCharacter":
        """Deserialize a :class:`CompactCharacter` from JSON string."""
        return cls.from_dict(json.loads(data))


__all__ = ["CompactCharacter"]
