from __future__ import annotations

from src.core.config import get_logger
from .partners import run_partners


class NeuronNetwork:
    """Minimal neuron network stub used for command processing.

    The real project may implement complex neuron activation here. For testing
    purposes we simply echo the command to demonstrate the processing flow.
    """

    def __init__(self) -> None:
        self.logger = get_logger(__name__)

    def process(self, command: str) -> str:
        """Process a command and return a textual response."""

        self.logger.debug("NeuronNetwork received command: %s", command)
        if command.startswith("partner:"):
            text = command.split(":", 1)[1].strip()
            return run_partners(text)
        return f"processed: {command}"


__all__ = ["NeuronNetwork"]
