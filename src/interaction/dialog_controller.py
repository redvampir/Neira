from __future__ import annotations

from enum import Enum, auto
from typing import Callable, Any

from rich.console import Console
from rich.panel import Panel


class DialogController:
    """Простейший контроллер диалога, задающий уточняющие вопросы."""

    class Step(Enum):
        """Шаги взаимодействия с пользователем."""

        WAITING_COMMAND = auto()
        WAITING_CLARIFICATION = auto()
        PROCESSING = auto()

    def __init__(
        self,
        neyra: Any,
        input_func: Callable[[str], str] | None = None,
        output_func: Callable[[str], None] | None = None,
        *,
        exit_command: str = "/exit",
        clarification_prompt: str = "Уточните, пожалуйста: ",
        use_color: bool = True,
    ) -> None:
        """Создаёт контроллер диалога.

        Args:
            neyra: Объект с методом ``process_command``.
            input_func: Функция получения ввода (по умолчанию ``input``).
            output_func: Функция вывода (по умолчанию ``rich`` console).
            exit_command: Команда завершения диалога.
            clarification_prompt: Подсказка для уточняющего вопроса.
            use_color: Включает цветной вывод. Отключите для монохромных терминалов.
        """

        self.neyra = neyra
        self.input_func = input_func or input
        self.console = Console(no_color=not use_color)
        self.output_func = output_func or self._render_output
        self.exit_command = exit_command
        self.clarification_prompt = clarification_prompt
        self.step: DialogController.Step = DialogController.Step.WAITING_COMMAND

    def interact(self) -> None:
        """Запускает цикл диалога."""

        while True:
            command = self.input_func("> ")
            if command.strip() == self.exit_command:
                break
            if not command.strip():
                continue

            self.step = DialogController.Step.WAITING_CLARIFICATION
            clarification = self.input_func(self.clarification_prompt)

            full_command = f"{command} {clarification}".strip()
            self.step = DialogController.Step.PROCESSING
            result = self.neyra.process_command(full_command)
            self.output_func(result)
            self.step = DialogController.Step.WAITING_COMMAND

    def _render_output(self, message: str) -> None:
        """Выводит ответ Нейры, выделяя важные части."""
        lower = message.lower()
        if "@" in message:
            self.console.print(Panel(message, style="cyan"))
        elif "эмоци" in lower:
            self.console.print(Panel(message, style="magenta"))
        elif any(word in lower for word in ["опис", "сцена"]):
            self.console.print(Panel(message, style="green"))
        else:
            self.console.print(message)


__all__ = ["DialogController"]
