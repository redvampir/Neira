import os
import sys

sys.path.append(os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..")))

from characters.rpg_character import RPGCharacter


def test_serialization_roundtrip():
    original = RPGCharacter(
        name="Hero",
        attributes={"strength": 10, "agility": 8},
        skills=["swordsmanship", "archery"],
        equipment=["sword", "bow"],
        status_effects=["poisoned"],
        ai_personality_type="aggressive",
        decision_patterns=["attack", "defend"],
        roleplay_style="heroic",
    )

    data = original.to_dict()
    recreated = RPGCharacter.from_dict(data)

    assert recreated == original
