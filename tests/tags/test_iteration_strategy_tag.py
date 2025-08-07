import pytest

from src.core.neyra_brain import Neyra
from src.iteration.strategy_manager import IterationStrategy


@pytest.mark.parametrize(
    "mode, iterations, spaces",
    [
        ("quick", 1, 0),
        ("standard", 3, 0),
        ("thorough", 5, 0),
        ("research", 8, 1),
    ],
)
def test_iteration_tag_selects_strategy(monkeypatch, mode, iterations, spaces):
    neyra = Neyra()
    captured = {}

    def fake_iterative_response(query: str, strategy: IterationStrategy | None = None):
        captured["strategy"] = strategy
        return "ok"

    monkeypatch.setattr(neyra, "iterative_response", fake_iterative_response)

    result = neyra.process_command(f"@Итерация: {mode}@")

    assert result == "ok"
    strategy = captured["strategy"]
    assert isinstance(strategy, IterationStrategy)
    assert strategy.max_iterations == iterations
    assert strategy.max_critical_spaces == spaces
