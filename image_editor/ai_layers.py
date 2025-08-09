"""Layer generation via external AI API."""

from __future__ import annotations

import os
from typing import Tuple

import numpy as np
import requests

from .layers import Layer


def generate_layer(prompt: str, size: Tuple[int, int] = (64, 64)) -> Layer:
    """Generate a new layer using an external service.

    The service URL can be provided via the ``AI_LAYER_API_URL`` environment
    variable. When not set, random noise is returned which keeps tests
    deterministic enough for shape checking.
    """

    api_url = os.getenv("AI_LAYER_API_URL")
    width, height = size
    if api_url:
        resp = requests.post(
            api_url,
            json={"prompt": prompt, "width": width, "height": height},
            timeout=30,
        )
        resp.raise_for_status()
        data = np.array(resp.json()["data"], dtype=np.uint8)
        data = data.reshape((height, width, 3))
    else:
        data = (np.random.rand(height, width, 3) * 255).astype(np.uint8)
    return Layer(name=prompt, content=data)
