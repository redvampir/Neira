"""Learning utilities and adaptive systems."""

from .learning_system import LearningSystem
from .error_analysis import classify_error, recommend_action
from .feedback import FeedbackInterface
from .knowledge_base import KnowledgeBase
from .online_updates import OnlineLearningEngine

__all__ = [
    "LearningSystem",
    "classify_error",
    "recommend_action",
    "FeedbackInterface",
    "KnowledgeBase",
    "OnlineLearningEngine",
]
