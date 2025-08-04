from __future__ import annotations

from enum import Enum, auto
from typing import Callable, Any


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
    ) -> None:
        """Создаёт контроллер диалога.

        Args:
            neyra: Объект с методом ``process_command``.
            input_func: Функция получения ввода (по умолчанию ``input``).
            output_func: Функция вывода (по умолчанию ``print``).
            exit_command: Команда завершения диалога.
            clarification_prompt: Подсказка для уточняющего вопроса.
        """

        self.neyra = neyra
        self.input_func = input_func or input
        self.output_func = output_func or print
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


__all__ = ["DialogController"]
