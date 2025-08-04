from src.interaction.dialog_controller import DialogController


class DummyNeyra:
    def __init__(self):
        self.commands = []
        self.state_at_call = None

    def process_command(self, command: str) -> str:
        self.commands.append(command)
        return "processed: " + command


def test_dialog_flow(capsys):
    inputs = iter(["привет", "добавь детали", "/exit"])
    neyra = DummyNeyra()
    controller = DialogController(neyra)
    states = []

    def fake_input(prompt: str) -> str:  # noqa: D401 - simple test helper
        states.append(controller.step)
        return next(inputs)

    controller.input_func = fake_input

    def patched_process(command: str) -> str:
        neyra.state_at_call = controller.step
        return DummyNeyra.process_command(neyra, command)

    neyra.process_command = patched_process  # type: ignore[method-assign]

    controller.interact()

    assert states == [
        DialogController.Step.WAITING_COMMAND,
        DialogController.Step.WAITING_CLARIFICATION,
        DialogController.Step.WAITING_COMMAND,
    ]
    assert neyra.state_at_call == DialogController.Step.PROCESSING
    assert neyra.commands == ["привет добавь детали"]
    captured = capsys.readouterr()
    assert "processed: привет добавь детали" in captured.out


def test_process_called_once():
    inputs = iter(["первая", "вторая", "/exit"])
    neyra = DummyNeyra()
    controller = DialogController(neyra, input_func=lambda _: next(inputs))
    controller.interact()
    assert neyra.commands == ["первая вторая"]
