"""Disable the local LLM by updating the configuration."""

from __future__ import annotations

import json
from pathlib import Path


def main() -> None:
    """Set ``model_type`` to ``none`` and remove ``model_path``."""
    config_path = Path(__file__).resolve().parents[1] / "config" / "llm_config.json"
    if config_path.exists():
        cfg = json.loads(config_path.read_text(encoding="utf-8"))
    else:
        cfg = {}

    cfg["model_type"] = "none"
    cfg.pop("model_path", None)

    config_path.write_text(
        json.dumps(cfg, ensure_ascii=False, indent=2) + "\n", encoding="utf-8"
    )
    print(f"LLM disabled in {config_path}")


if __name__ == "__main__":  # pragma: no cover - manual invocation helper
    main()
