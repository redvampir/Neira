"""Utilities for uploading and downloading plugin packages.

The project uses a very small plugin system where Python modules are placed in
``plugins/``. Some tooling requires transferring these plugins as ``.zip``
packages. The helpers in this module provide a lightweight API for saving and
retrieving such archives without introducing additional dependencies.
"""

from __future__ import annotations

from pathlib import Path
import shutil


def upload_package(zip_path: Path, plugin_dir: Path | str = "plugins") -> Path:
    """Store ``zip_path`` inside ``plugin_dir``.

    Parameters
    ----------
    zip_path:
        Path to an existing ``.zip`` file.
    plugin_dir:
        Destination directory. Created when missing.

    Returns
    -------
    Path
        Location of the stored package.
    """

    plugin_dir = Path(plugin_dir)
    plugin_dir.mkdir(parents=True, exist_ok=True)
    destination = plugin_dir / Path(zip_path).name
    shutil.copy(zip_path, destination)
    return destination


def download_package(name: str, plugin_dir: Path | str = "plugins") -> bytes:
    """Return the binary contents of a stored plugin package.

    ``name`` may be provided with or without the ``.zip`` suffix.
    """

    plugin_dir = Path(plugin_dir)
    path = plugin_dir / name
    if path.suffix != ".zip":
        path = path.with_suffix(".zip")
    return path.read_bytes()


__all__ = ["upload_package", "download_package"]
