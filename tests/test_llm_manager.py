from __future__ import annotations

"""Tests for :mod:`src.llm.manager`."""

from src.llm.manager import LLMManager, Task
from src.llm.base_llm import BaseLLM


class DummyLLM(BaseLLM):
    model_name = "dummy"

    def __init__(self, response: str, available: bool = True) -> None:
        self.response = response
        self.available = available

    def generate(self, prompt: str, max_tokens: int = 512) -> str:  # pragma: no cover - simple dummy
        return f"{self.response}:{prompt}"

    def is_available(self) -> bool:
        return self.available


def test_selection_and_prompt_adaptation() -> None:
    manager = LLMManager()
    fast = DummyLLM("fast")
    accurate = DummyLLM("accurate")

    manager.register_model("fast", fast, speed=10, cost=1, accuracy=0.5)
    manager.register_model(
        "accurate", accurate, speed=1, cost=2, accuracy=0.9, prompt_adapter=str.upper
    )

    # Request explicitly asking for speed selects fast model
    name, model, adapted = manager.select_model(Task(prompt="hi", request_type="fast"))
    assert name == "fast"
    assert model is fast
    assert adapted == "hi"

    # General request prefers accuracy and applies adapter
    name, model, adapted = manager.select_model(Task(prompt="hello"))
    assert name == "accurate"
    assert adapted == "HELLO"

    # Long prompt should favour speed
    long_prompt = "x" * 101
    name, _, _ = manager.select_model(Task(prompt=long_prompt))
    assert name == "fast"

    # If fast becomes unavailable, accurate is chosen
    fast.available = False
    name, _, _ = manager.select_model(Task(prompt="short"))
    assert name == "accurate"


def test_ensemble_and_learning_integration() -> None:
    manager = LLMManager()
    fast = DummyLLM("fast")
    accurate = DummyLLM("accurate")

    manager.register_model("fast", fast, speed=10, cost=1, accuracy=0.5)
    manager.register_model("accurate", accurate, speed=1, cost=2, accuracy=0.9)

    result = manager.generate(Task(prompt="prompt"), ensemble=True)
    assert result == "accurate:prompt"
    assert manager.learning_system.experience_buffer  # interaction recorded
    interaction = manager.learning_system.experience_buffer[0]
    assert interaction["context"]["model"] == "accurate"
    assert interaction["response"] == "accurate:prompt"
