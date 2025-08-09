"""Tests for the scenario engine."""

from game.scenario_engine import ScenarioEngine


def test_events_are_triggered_on_correct_turns() -> None:
    engine = ScenarioEngine()
    results = []

    def event(x: str) -> None:
        results.append(x)

    engine.schedule(event, 1, "a")
    engine.schedule(event, 2, "b")

    engine.run_turn()
    assert results == ["a"]

    engine.run_turn()
    assert results == ["a", "b"]


def test_clear_resets_state() -> None:
    engine = ScenarioEngine()
    engine.schedule(lambda: None, 1)
    engine.clear()
    assert engine.current_turn == 0
    engine.run_turn()
    assert engine.current_turn == 1
