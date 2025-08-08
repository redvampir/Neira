"""Simple API for notifying UI about progress."""
from __future__ import annotations

from typing import Optional

from src.core.config import get_logger

logger = get_logger(__name__)


def update_progress(stage: str, iteration: Optional[int] = None) -> None:
    """Log an update about the current progress stage.

    Parameters
    ----------
    stage:
        Name of the stage that is currently being executed.
    iteration:
        Optional iteration number for iterative processes.
    """
    if iteration is not None:
        logger.info("UI progress - %s (iteration %s)", stage, iteration)
    else:
        logger.info("UI progress - %s", stage)
