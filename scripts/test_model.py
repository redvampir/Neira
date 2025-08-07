"""Quick check for the local LLM configuration.

This script reads ``config/llm_config.json``, instantiates the configured
LLM via :class:`~src.llm.base_llm.LLMFactory` and runs a tiny test prompt.
It gracefully handles environments where the heavyweight dependencies are not
installed.
"""

from __future__ import annotations

import json
import sys
from pathlib import Path


# Ensure the project root is on ``sys.path`` so that ``src`` can be imported
ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.append(str(ROOT))

from src.llm import LLMFactory  # noqa: E402  (import after sys.path tweak)


def main() -> None:
    """Load configuration and run a sample prompt."""

    config_path = ROOT / "config" / "llm_config.json"
    cfg = json.loads(config_path.read_text(encoding="utf-8"))
    model_type = cfg.get("model_type", "mistral")
    model_path = cfg.get("model_path")
    max_tokens = int(cfg.get("max_tokens", 128))

    llm = LLMFactory.create(model_type, model_path=model_path)
    prompt = "Привет! Расскажи что-нибудь интересное."

    try:
        response = llm.generate(prompt, max_tokens=max_tokens)
    except Exception as exc:  # pragma: no cover - depends on external model
        print(f"LLM is not available: {exc}")
        return

    print("Model response:")
    print(response)


if __name__ == "__main__":  # pragma: no cover - manual invocation helper
    main()
