import json
from pathlib import Path
from typing import List

PROGRESS_FILE = Path("userdata/tutorial_progress.json")


class Tutorial:
    """Simple tutorial with a set of lesson scenes."""

    def __init__(self) -> None:
        self.lessons: List[str] = [
            "Урок 1. Добро пожаловать в обучение!",
            "Урок 2. Основы общения с Нейрой.",
            "Урок 3. Завершение обучения.",
        ]
        self.index = self._load_progress()

    # ------------------------------------------------------------------
    def _load_progress(self) -> int:
        """Load current lesson index from progress file."""
        if PROGRESS_FILE.exists():
            try:
                with PROGRESS_FILE.open("r", encoding="utf-8") as fh:
                    data = json.load(fh)
                    return int(data.get("current_lesson", 0))
            except (json.JSONDecodeError, OSError, ValueError):
                pass
        # default progress
        PROGRESS_FILE.parent.mkdir(parents=True, exist_ok=True)
        self._save_progress(0)
        return 0

    def _save_progress(self, value: int) -> None:
        PROGRESS_FILE.parent.mkdir(parents=True, exist_ok=True)
        with PROGRESS_FILE.open("w", encoding="utf-8") as fh:
            json.dump({"current_lesson": value}, fh, ensure_ascii=False, indent=2)

    # ------------------------------------------------------------------
    def current(self) -> str:
        """Return text of the current lesson."""
        return self.lessons[self.index]

    def next(self) -> str:
        """Advance to the next lesson and return its text."""
        if self.index < len(self.lessons) - 1:
            self.index += 1
            self._save_progress(self.index)
        return self.current()

    def previous(self) -> str:
        """Return to the previous lesson."""
        if self.index > 0:
            self.index -= 1
            self._save_progress(self.index)
        return self.current()

    # ------------------------------------------------------------------
    def run(self) -> None:
        """Run simple CLI navigation through lessons."""
        print(self.current())
        while True:
            cmd = input("(n) следующий, (p) предыдущий, (q) выход: ").strip().lower()
            if cmd == "n":
                print(self.next())
            elif cmd == "p":
                print(self.previous())
            elif cmd == "q":
                break
            else:
                print("Неизвестная команда")
