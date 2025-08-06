import pytest

from src.iteration.strategy_manager import AdaptiveIterationManager
from src.iteration.iteration_controller import IterationController


@pytest.mark.parametrize(
    "preset, expected_iterations, expected_spaces",
    [
        ("quick", 1, 0),
        ("standard", 3, 0),
        ("thorough", 5, 0),
        ("research", 8, 1),
    ],
)
def test_presets_define_limits(preset, expected_iterations, expected_spaces):
    manager = AdaptiveIterationManager(preset)
    assert manager.max_iterations == expected_iterations
    assert manager.max_critical_spaces == expected_spaces


def test_unknown_preset_raises():
    with pytest.raises(ValueError):
        AdaptiveIterationManager("unknown")


def test_controller_applies_strategy():
    controller = IterationController(strategy="research")
    assert controller.max_iterations == 8
    assert controller.max_critical_spaces == 1
