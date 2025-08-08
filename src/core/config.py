from __future__ import annotations

"""Application configuration and logging setup.

The module defines dataclasses describing configuration schema for the
project. Configuration values can be supplied via environment variables or
configuration files stored in the :mod:`config` directory.
"""

import json
import logging
import os
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any

import yaml
from dotenv import load_dotenv

# Load environment variables from a .env file if present
load_dotenv()


@dataclass
class PathSettings:
    """Filesystem related settings."""

    base_dir: Path = Path(os.getenv("NEIRA_BASE_DIR", Path(__file__).resolve().parents[2]))
    config_dir: Path = Path(os.getenv("NEIRA_CONFIG_DIR", base_dir / "config"))
    logs_dir: Path = Path(os.getenv("NEIRA_LOG_DIR", base_dir / "logs"))


@dataclass
class LoggingSettings:
    """Logging configuration."""

    level: str = os.getenv("NEIRA_LOG_LEVEL", "INFO")
    format: str = os.getenv(
        "NEIRA_LOG_FORMAT",
        "%(asctime)s - %(name)s - %(levelname)s - %(message)s",
    )
    file: str = os.getenv("NEIRA_LOG_FILE", "neyra.log")


@dataclass
class Settings:
    """Complete application settings."""

    paths: PathSettings = field(default_factory=PathSettings)
    logging: LoggingSettings = field(default_factory=LoggingSettings)


settings = Settings()


def setup_logging() -> None:
    """Configure the root logger according to :data:`settings`."""

    logs_dir = settings.paths.logs_dir
    logs_dir.mkdir(parents=True, exist_ok=True)
    log_file = logs_dir / settings.logging.file
    level = getattr(logging, settings.logging.level.upper(), logging.INFO)
    logging.basicConfig(
        level=level,
        format=settings.logging.format,
        handlers=[
            logging.FileHandler(log_file, encoding="utf-8"),
            logging.StreamHandler(),
        ],
        force=True,
    )


def get_logger(name: str) -> logging.Logger:
    """Return a module-level logger following the global configuration."""

    return logging.getLogger(name)


def load_config_file(name: str) -> Any:
    """Load a configuration file from the configured directory.

    Parameters
    ----------
    name:
        File name within :attr:`PathSettings.config_dir`.

    Returns
    -------
    Any
        Parsed configuration data.
    """

    path = settings.paths.config_dir / name
    if not path.exists():  # pragma: no cover - simple guard
        raise FileNotFoundError(f"Config file {name} not found in {settings.paths.config_dir}")
    text = path.read_text(encoding="utf-8")
    if path.suffix.lower() in {".yml", ".yaml"}:
        return yaml.safe_load(text)
    if path.suffix.lower() == ".json":
        return json.loads(text)
    raise ValueError(f"Unsupported config format: {path.suffix}")


# Configure logging as soon as the module is imported
setup_logging()


__all__ = [
    "settings",
    "setup_logging",
    "get_logger",
    "load_config_file",
    "Settings",
    "PathSettings",
    "LoggingSettings",
]
