"""Memory subsystem classes and utilities with lazy loading."""

from importlib import import_module
from typing import Any

__all__ = [
    "CharacterMemory",
    "EmotionalMemory",
    "StoryTimeline",
    "WorldAtlas",
    "WorldMemory",
    "StyleMemory",
    "MemoryIndex",
    "EmbeddingMemory",
    "WeightedMemory",
    "MultiGridMemory",
    "LazyMemoryLoader",
    "KnowledgeGraph",
    "knowledge_graph",
]

_MODULES = {
    "CharacterMemory": "character_memory",
    "EmotionalMemory": "emotional_memory",
    "StoryTimeline": "story_timeline",
    "WorldAtlas": "world_atlas",
    "WorldMemory": "world_memory",
    "StyleMemory": "style_memory",
    "MemoryIndex": "index",
    "EmbeddingMemory": "embedding_memory",
    "WeightedMemory": "weighted",
    "MultiGridMemory": "multi_grid",
    "LazyMemoryLoader": "lazy_loader",
    "KnowledgeGraph": "knowledge_graph",
    "knowledge_graph": "knowledge_graph",
}


def __getattr__(name: str) -> Any:  # pragma: no cover - simple proxy
    module_name = _MODULES.get(name)
    if module_name is None:
        raise AttributeError(f"module {__name__} has no attribute {name}")
    module = import_module(f".{module_name}", __name__)
    return getattr(module, name)

