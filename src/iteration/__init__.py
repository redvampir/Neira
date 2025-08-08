"""Iteration utilities for Neyra."""

from .draft_generator import DraftGenerator
from .gap_analyzer import GapAnalyzer, KnowledgeGap
try:  # pragma: no cover - optional dependency during tests
    from .deep_searcher import DeepSearcher
except Exception:  # noqa: BLE001 - fallback when requests is missing
    DeepSearcher = None  # type: ignore
from .response_enhancer import ResponseEnhancer, IntegrationType
from .feedback_learner import FeedbackLearner
from .iteration_controller import IterationController
from .iterative_generator import IterativeGenerator
from .strategy_manager import AdaptiveIterationManager, IterationStrategy
from .resource_iterator import ResourceAwareIterator
from .low_resource_optimizer import LowResourceOptimizer
from .smart_cache import SmartCache
from .metrics import similarity, length, corrected_errors, log_metrics
from .memory_inspector import MemoryInspector
from .token_budget_manager import TokenBudgetManager

__all__ = [
    "DraftGenerator",
    "GapAnalyzer",
    "KnowledgeGap",
    "DeepSearcher",
    "ResponseEnhancer",
    "FeedbackLearner",
    "IntegrationType",
    "IterationController",
    "IterativeGenerator",
    "AdaptiveIterationManager",
    "IterationStrategy",
    "ResourceAwareIterator",
    "LowResourceOptimizer",
    "SmartCache",
    "MemoryInspector",
    "TokenBudgetManager",
    "similarity",
    "length",
    "corrected_errors",
    "log_metrics",
]
