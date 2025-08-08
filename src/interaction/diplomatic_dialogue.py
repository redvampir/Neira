from __future__ import annotations

from typing import Iterable, Tuple

from src.utils.source_manager import SourceManager


class DiplomaticDialogue:
    """Диалог расследования с обменом источниками."""

    start_template = "Начато расследование: {topic}"
    progress_template = "Источники:\n{sources}"

    def __init__(self, source_manager: SourceManager | None = None) -> None:
        self.source_manager = source_manager or SourceManager()

    def start_investigation(
        self,
        topic: str,
        sources: Iterable[Tuple[str, str, float]] = (),
    ) -> str:
        """Регистрирует источники и запускает расследование."""

        for summary, path, reliability in sources:
            self.source_manager.register(summary, path, reliability)
        return self.start_template.format(topic=topic)

    def continue_research(self) -> str:
        """Возвращает список известных источников."""

        entries = self.source_manager.all()
        if not entries:
            return "Источники не найдены"
        lines = [f"[{i + 1}] {s.summary} ({s.path})" for i, s in enumerate(entries)]
        return self.progress_template.format(sources="\n".join(lines))


__all__ = ["DiplomaticDialogue"]
