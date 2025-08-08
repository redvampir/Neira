"""Download the Qwen Coder GGUF model and update configuration.

This script retrieves the Qwen‑2.5 Coder **1.5B** Instruct model in GGUF
format and stores it under ``models/qwen``. After a successful download,
``config/llm_config.json`` is created or updated so that subsequent runs of
Neyra use the new model.

The chosen defaults assume roughly 8 GB of system RAM; tweak ``max_tokens`` if
your hardware differs.
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
# Primary and mirror locations for the model
MODEL_URLS = [
    (
        "https://huggingface.co/Qwen/Qwen2.5-Coder-1.5B-Instruct-GGUF/resolve/main/"
        "Qwen2.5-Coder-1.5B-Instruct-Q4_K_M.gguf"
    ),
    (
        "https://huggingface.co/bartowski/Qwen2.5-Coder-1.5B-Instruct-GGUF/resolve/main/"
        "Qwen2.5-Coder-1.5B-Instruct-Q4_K_M.gguf"
    ),
]
MODEL_DIR = ROOT / "models" / "qwen"
MODEL_PATH = MODEL_DIR / "Qwen2.5-Coder-1.5B-Instruct-Q4_K_M.gguf"
CONFIG_PATH = ROOT / "config" / "llm_config.json"
DEFAULT_MAX_TOKENS = 1024
MODEL_TYPE = "qwen_coder"


def download() -> None:
    """Download the GGUF model if it is not already present."""

    MODEL_DIR.mkdir(parents=True, exist_ok=True)
    if MODEL_PATH.exists():
        print(f"Model already exists at {MODEL_PATH}")
        return

    for url in MODEL_URLS:
        try:
            with requests.get(url, stream=True) as r:
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
            break
        except requests.HTTPError as exc:
            print(f"HTTP error for {url}: {exc}")
        except requests.RequestException as exc:
            print(f"Network error for {url}: {exc}")
    else:
        print("Error: could not download the model from any provided URL.")


def update_config() -> None:
    """Write or update ``llm_config.json`` to use the downloaded model."""

    cfg = {}
    if CONFIG_PATH.exists():
        try:
            cfg = json.loads(CONFIG_PATH.read_text(encoding="utf-8"))
        except json.JSONDecodeError:
            print(f"Warning: {CONFIG_PATH} contains invalid JSON, overwriting")

    cfg.update(
        {
            "model_type": MODEL_TYPE,
            "model_path": str(MODEL_PATH),
            "max_tokens": DEFAULT_MAX_TOKENS,
        }
    )
    CONFIG_PATH.write_text(
        json.dumps(cfg, indent=2, ensure_ascii=False), encoding="utf-8"
    )
    print(
        f"Updated config to use {MODEL_PATH} with {DEFAULT_MAX_TOKENS} max tokens"
    )


def main() -> None:
    download()
    update_config()


if __name__ == "__main__":  # pragma: no cover - manual execution helper
    sys.exit(main())
