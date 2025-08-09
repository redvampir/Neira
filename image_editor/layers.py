from __future__ import annotations

"""Core layer representation with mask and effect support."""

from dataclasses import dataclass, field
from typing import Callable, List, Optional

import numpy as np

Effect = Callable[[np.ndarray], np.ndarray]


@dataclass
class Layer:
    """Represents an image layer.

    Attributes:
        name: Human readable layer name.
        content: Numpy array of shape (H, W, C).
        mask: Optional alpha mask with shape (H, W) or (H, W, 1).
        effects: List of callables applied sequentially to the content.
        vector_data: Optional SVG snippets representing vector shapes.
    """

    name: str
    content: np.ndarray
    mask: Optional[np.ndarray] = None
    effects: List[Effect] = field(default_factory=list)
    vector_data: Optional[List[str]] = None

    def apply_mask(self, data: np.ndarray) -> np.ndarray:
        """Apply layer mask to data."""
        if self.mask is None:
            return data
        mask = self.mask
        if mask.ndim == 2:
            mask = mask[..., np.newaxis]
        if mask.max() > 1:
            mask = mask / 255.0
        return (data * mask).astype(data.dtype)

    def apply_effects(self, data: np.ndarray) -> np.ndarray:
        """Apply registered effects to data."""
        for effect in self.effects:
            data = effect(data)
        return data

    def render(self) -> np.ndarray:
        """Return content with mask and effects applied."""
        data = self.apply_mask(self.content.copy())
        data = self.apply_effects(data)
        return data


def invert_effect(data: np.ndarray) -> np.ndarray:
    """Simple color inversion effect."""
    return 255 - data


def grayscale_effect(data: np.ndarray) -> np.ndarray:
    """Convert to grayscale keeping three channels."""
    gray = data.mean(axis=2, keepdims=True)
    return np.repeat(gray, 3, axis=2).astype(data.dtype)


def generate_ai_layer(prompt: str, size: tuple[int, int] = (64, 64)) -> Layer:
    """Generate a layer using the AI API.

    This is a thin wrapper that delegates to :mod:`image_editor.ai_layers`.
    """

    from .ai_layers import generate_layer

    return generate_layer(prompt, size)
