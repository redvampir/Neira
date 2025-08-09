"""Utilities and base classes for visual mode language parsing."""

from .base import LanguageParser  # re-export for convenience
from . import utils

__all__ = ["LanguageParser", "utils"]
