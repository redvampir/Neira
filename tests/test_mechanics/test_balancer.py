"""Tests for the GameBalancer."""

from __future__ import annotations

from src.mechanics.balancer import GameBalancer


def test_monitor_encounter_difficulty_returns_average() -> None:
    balancer = GameBalancer()
    assert balancer.monitor_encounter_difficulty(5) == 5
    assert balancer.monitor_encounter_difficulty(7) == 6


def test_adjust_challenge_level_moves_toward_target() -> None:
    balancer = GameBalancer()
    balancer.monitor_encounter_difficulty(5)
    balancer.monitor_encounter_difficulty(7)
    level = balancer.adjust_challenge_level(8)
    assert level == 2
    assert balancer.adjust_challenge_level(5) == 1


def test_ensure_spotlight_distribution_checks_balance() -> None:
    balancer = GameBalancer()
    assert balancer.ensure_spotlight_distribution(["Alice", "Bob"])
    assert balancer.ensure_spotlight_distribution(["Alice"])
    assert not balancer.ensure_spotlight_distribution(["Alice"])
