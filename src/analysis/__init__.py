from .self_corrector import SelfCorrector, SuggestionChooser
from .verification_system import (
    VerificationSystem,
    VerificationResult,
    verify_fact,
    verify_claim,
)
from .uncertainty_manager import UncertaintyManager
from .timeline_checker import TimelineChecker
from .post_processor import PostProcessor, run_post_processors
from .grammar_proofreader import GrammarProofreader
from .candidate_generator import CandidateGenerator
from .candidate_selector import CandidateSelector
from .reasoning_planner import ReasoningPlanner, ReasoningStep

from src.core.state_manager import StateManager

POST_PROCESSOR_REGISTRY = {
    "GrammarProofreader": GrammarProofreader,
}

__all__ = [
    "SelfCorrector",
    "SuggestionChooser",
    "VerificationSystem",
    "VerificationResult",
    "UncertaintyManager",
    "TimelineChecker",
    "PostProcessor",
    "run_post_processors",
    "GrammarProofreader",
    "CandidateGenerator",
    "CandidateSelector",
    "POST_PROCESSOR_REGISTRY",
    "verify_fact",
    "verify_claim",
    "ReasoningPlanner",
    "ReasoningStep",
    "analysis_state",
    "begin",
    "commit",
    "rollback",
]

# Global state manager for analysis subsystem to allow transactional
# operations when multiple analysis components interact.
analysis_state = StateManager()


def begin() -> None:
    """Create a snapshot of the analysis state."""

    analysis_state.begin()


def commit() -> None:
    """Commit changes performed since the last :func:`begin`."""

    analysis_state.commit()


def rollback() -> None:
    """Restore the analysis state from the last snapshot."""

    analysis_state.rollback()
