"""Tests for the RPGLearningSystem."""

from __future__ import annotations

from src.learning.rpg_learning import RPGLearningSystem


def test_analyze_player_reactions_counts_labels() -> None:
    system = RPGLearningSystem()
    reactions = ["happy", "sad", "happy"]
    assert system.analyze_player_reactions(reactions) == {"happy": 2, "sad": 1}


def test_improve_master_skills_accumulates_feedback() -> None:
    system = RPGLearningSystem()
    system.improve_master_skills({"craft": 2})
    updated = system.improve_master_skills({"craft": 3, "magic": 1})
    assert updated == {"craft": 5, "magic": 1}


def test_adapt_personality_behaviors_clamps_values() -> None:
    system = RPGLearningSystem()
    system.personality_traits = {"bold": 0.9}
    traits = system.adapt_personality_behaviors({"bold": 0.5, "cautious": -0.2})
    assert traits["bold"] == 1.0
    assert traits["cautious"] == -0.2


def test_learn_from_successful_scenarios_records_unique() -> None:
    system = RPGLearningSystem()
    system.learn_from_successful_scenarios(["quest1", "quest2"])
    scenarios = system.learn_from_successful_scenarios(["quest2", "quest3"])
    assert scenarios == ["quest1", "quest2", "quest3"]
