"""Tests for :mod:`src.llm.qwen_coder_interface`."""
from __future__ import annotations

from typing import Any, Dict

from src.llm import qwen_coder_interface as qci


def test_lazy_loading_and_generation(monkeypatch) -> None:
    """The model should load lazily and honour generation parameters."""

    calls: Dict[str, Dict[str, Any]] = {}

    class DummyLlama:
        def __init__(self, **kwargs: Any) -> None:  # pragma: no cover - simple dummy
            calls["init"] = kwargs

        def __call__(self, prompt: str, **kwargs: Any) -> Dict[str, Any]:
            calls["call"] = {"prompt": prompt, **kwargs}
            return {"choices": [{"text": "response"}]}

    monkeypatch.setattr(qci, "Llama", DummyLlama)

    llm = qci.QwenCoderLLM(
        "model.gguf",
        n_gpu_layers=1,
        n_ctx=256,
        n_batch=8,
        use_mmap=False,
        use_mlock=True,
        seed=123,
    )

    assert not llm.is_available()

    text = llm.generate(
        "Hello",
        max_tokens=5,
        temperature=0.6,
        top_p=0.7,
        repeat_penalty=1.2,
        stop=["END"],
    )

    assert text == "response"
    assert llm.is_available()

    # Ensure correct initialisation parameters were passed to Llama
    assert calls["init"] == {
        "model_path": "model.gguf",
        "n_gpu_layers": 1,
        "n_ctx": 256,
        "n_batch": 8,
        "use_mmap": False,
        "use_mlock": True,
        "seed": 123,
        "verbose": False,
    }

    # And generation parameters were forwarded correctly
    assert calls["call"] == {
        "prompt": "Hello",
        "max_tokens": 5,
        "temperature": 0.6,
        "top_p": 0.7,
        "repeat_penalty": 1.2,
        "stop": ["END"],
    }

