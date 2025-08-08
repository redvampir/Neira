# -*- coding: utf-8 -*-
"""Generic downloader for arbitrary GGUF models.

The script fetches a GGUF model from a provided URL, stores it under
``models/<model_type>`` and updates ``config/llm_config.json`` so that
subsequent runs of Neyra use the new model.
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

import requests
from tqdm import tqdm


ROOT = Path(__file__).resolve().parents[1]
CONFIG_PATH = ROOT / "config" / "llm_config.json"


def download(model_type: str, url: str) -> None:
    """Download a GGUF model and update the configuration.

    Parameters
    ----------
    model_type:
        Identifier for the model, e.g. ``mistral`` or ``qwen_coder``.
    url:
        Direct link to the GGUF model file.
    """

    model_dir = ROOT / "models" / model_type
    model_dir.mkdir(parents=True, exist_ok=True)

    filename = url.split("/")[-1]
    model_path = model_dir / filename

    if not model_path.exists():
        with requests.get(url, stream=True) as response:
            response.raise_for_status()
            total = int(response.headers.get("content-length", 0))
            with open(model_path, "wb") as file, tqdm(
                total=total, unit="B", unit_scale=True, desc="Downloading"
            ) as pbar:
                for chunk in response.iter_content(chunk_size=1024 * 1024):
                    if chunk:
                        file.write(chunk)
                        pbar.update(len(chunk))
        print(f"Downloaded model to {model_path}")
    else:
        print(f"Model already exists at {model_path}")

    config: dict[str, str] = {}
    if CONFIG_PATH.exists():
        try:
            config = json.loads(CONFIG_PATH.read_text(encoding="utf-8"))
        except json.JSONDecodeError:
            print(f"Warning: {CONFIG_PATH} contains invalid JSON, overwriting")

    config.update({"model_type": model_type, "model_path": str(model_path)})
    CONFIG_PATH.write_text(
        json.dumps(config, indent=2, ensure_ascii=False), encoding="utf-8"
    )
    print(f"Updated config to use {model_path}")


def main(argv: list[str] | None = None) -> None:
    """CLI entry point."""

    parser = argparse.ArgumentParser(
        description="Download a GGUF model and update configuration"
    )
    parser.add_argument(
        "model_type", help="Model type identifier, e.g. mistral or qwen_coder"
    )
    parser.add_argument("url", help="Direct URL to the GGUF model file")
    args = parser.parse_args(argv)
    download(args.model_type, args.url)


if __name__ == "__main__":  # pragma: no cover - manual execution helper
    sys.exit(main())

