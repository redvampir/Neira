
"""Download the Qwen‑2.5 GGUF model and update configuration.

The script fetches a lightweight variant of the Qwen‑2.5 Instruct model in
GGUF format and stores it under ``models/qwen``. After a successful download
``config/llm_config.json`` is updated so that subsequent runs of Neyra use the
new model.
"""

from __future__ import annotations

import json
import sys
from pathlib import Path

import requests
from tqdm import tqdm


# ---------------------------------------------------------------------------
# Paths and constants
ROOT = Path(__file__).resolve().parents[1]
MODEL_URL = (
    "https://huggingface.co/Qwen/Qwen2.5-0.5B-Instruct-GGUF/resolve/main/"
    "Qwen2.5-0.5B-Instruct-Q4_K_M.gguf"
)
MODEL_DIR = ROOT / "models" / "qwen"
MODEL_PATH = MODEL_DIR / "Qwen2.5-0.5B-Instruct-Q4_K_M.gguf"
CONFIG_PATH = ROOT / "config" / "llm_config.json"


def download() -> None:
    """Download the GGUF model if it is not already present."""

    MODEL_DIR.mkdir(parents=True, exist_ok=True)
    if MODEL_PATH.exists():
        print(f"Model already exists at {MODEL_PATH}")
        return

    with requests.get(MODEL_URL, stream=True) as r:
        r.raise_for_status()
        total = int(r.headers.get("content-length", 0))
        with open(MODEL_PATH, "wb") as f, tqdm(
            total=total, unit="B", unit_scale=True, desc="Downloading"
        ) as pbar:
            for chunk in r.iter_content(chunk_size=1024 * 1024):
                if chunk:
                    f.write(chunk)
                    pbar.update(len(chunk))

    print("Download complete.")


def update_config() -> None:
    """Point ``llm_config.json`` to the downloaded model."""

    if not CONFIG_PATH.exists():
        print(f"Config file {CONFIG_PATH} not found")
        return

    cfg = json.loads(CONFIG_PATH.read_text(encoding="utf-8"))
    cfg["model_path"] = str(MODEL_PATH)
    CONFIG_PATH.write_text(
        json.dumps(cfg, indent=2, ensure_ascii=False), encoding="utf-8"
    )
    print(f"Updated config to use {MODEL_PATH}")


def main() -> None:
    download()
    update_config()


if __name__ == "__main__":  # pragma: no cover - manual execution helper
    sys.exit(main())

