from __future__ import annotations

from typing import Any, Dict

from src.llm import mistral_interface as mi


def test_low_vram_cpu_fallback(monkeypatch) -> None:
    """When VRAM is critically low the LLM should reduce settings and use CPU."""

    calls: Dict[str, Dict[str, Any]] = {}

    monkeypatch.setattr(mi, "get_available_vram", lambda: 0.01)

    class DummyLlama:
        def __init__(self, **kwargs: Any) -> None:  # pragma: no cover - simple dummy
            calls["init"] = kwargs

        def __call__(self, prompt: str, **kwargs: Any) -> Dict[str, Any]:
            return {"choices": [{"text": "result"}]}

    monkeypatch.setattr(mi, "Llama", DummyLlama)

    llm = mi.MistralLLM("model.gguf", n_gpu_layers=10, n_ctx=1024, n_batch=64)
    text = llm.generate("Hi")

    assert text == "result"
    assert calls["init"]["n_ctx"] == 128
    assert calls["init"]["n_batch"] == 1
    assert calls["init"]["n_gpu_layers"] == 0
