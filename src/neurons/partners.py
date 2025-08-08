"""Orchestrator for partner neurons."""

from __future__ import annotations

from .factory import NeuronFactory


def run_partners(text: str) -> str:
    """Activate creative and logical partner neurons sequentially."""

    creative = NeuronFactory.create("creative_partner", id="creative")
    intermediate = creative.process(text)
    logical = NeuronFactory.create("logical_partner", id="logical")
    return logical.process(intermediate)


__all__ = ["run_partners"]
