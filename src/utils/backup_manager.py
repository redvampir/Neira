"""Simple backup utilities for the desktop application."""

from __future__ import annotations

from datetime import datetime
from pathlib import Path
import shutil
from typing import Union


class BackupManager:
    """Manage periodic backups of project files."""

    def __init__(self, backup_dir: Union[str, Path]) -> None:
        self.backup_dir = Path(backup_dir)
        self.backup_dir.mkdir(parents=True, exist_ok=True)

    def auto_backup(self, file_path: Union[str, Path]) -> Path:
        """Create a timestamped backup of ``file_path``."""
        src = Path(file_path)
        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        dest = self.backup_dir / f"{src.stem}_{timestamp}{src.suffix}"
        shutil.copy2(src, dest)
        return dest

    def restore_from_backup(self, backup_path: Union[str, Path], target_path: Union[str, Path]) -> None:
        """Restore ``backup_path`` to ``target_path``."""
        shutil.copy2(backup_path, target_path)
