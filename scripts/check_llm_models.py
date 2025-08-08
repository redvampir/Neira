"""Verify locally available LLM models and update configuration.

The script looks for ``*.gguf`` files in the predefined model directories and
writes ``config/llm_config.json`` with the detected model when possible.  When
the configuration already points to an existing model file a short message is
emitted instead.
"""

from __future__ import annotations

import json
import sys
from pathlib import Path


# Project root ---------------------------------------------------------------
ROOT = Path(__file__).resolve().parents[1]
MODEL_DIRS = {
    "qwen": ROOT / "models" / "qwen",
    "mistral": ROOT / "models" / "mistral",
}
CONFIG_PATH = ROOT / "config" / "llm_config.json"

# Ensure imports from ``src`` work when run as a script
if str(ROOT) not in sys.path:
    sys.path.append(str(ROOT))

from src.llm import LLMFactory  # noqa: E402  (import after sys.path tweak)


def _backend_name(name: str) -> str:
    """Return the backend identifier used by :class:`LLMFactory`.

    Currently ``qwen`` models are registered as ``qwen_coder`` within the
    factory.  For other names the identity mapping is used.
    """

    if name in LLMFactory._registry:  # type: ignore[attr-defined]
        return name
    return f"{name}_coder"


def main() -> None:
    """Check available models and update ``llm_config.json`` accordingly."""

    # Existing configuration takes precedence
    if CONFIG_PATH.exists():
        try:
            cfg = json.loads(CONFIG_PATH.read_text(encoding="utf-8"))
        except json.JSONDecodeError:
            cfg = {}
        model_path = cfg.get("model_path")
        if model_path and Path(model_path).exists():
            print("модель подключена")
            return

    for name, model_dir in MODEL_DIRS.items():
        files = list(model_dir.glob("*.gguf"))
        if len(files) != 1:
            script = "download_qwen_coder.py" if name == "qwen" else f"download_{name}.py"
            print(
                f"В каталоге {model_dir} не найдено единственного GGUF-файла. "
                f"Запустите python scripts/{script}"
            )
            continue

        backend = _backend_name(name)
        llm_cls = LLMFactory._registry.get(backend)  # type: ignore[attr-defined]
        if llm_cls is None or not llm_cls.is_backend_available():
            print(f"Backend {backend} недоступен")
            continue

        cfg = {"model_type": name, "model_path": str(files[0])}
        CONFIG_PATH.parent.mkdir(parents=True, exist_ok=True)
        CONFIG_PATH.write_text(
            json.dumps(cfg, indent=2, ensure_ascii=False), encoding="utf-8"
        )
        print(f"Конфигурация обновлена: {cfg}")
        return

    print("Подходящая модель не найдена")


if __name__ == "__main__":  # pragma: no cover - manual invocation helper
    main()
