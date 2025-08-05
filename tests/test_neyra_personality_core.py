from src.core.neyra_personality import NeyraPersonalityCore


def test_get_trait_defaults_to_zero() -> None:
    personality = NeyraPersonalityCore()
    assert personality.get_trait("empathy") == 0.0


def test_apply_trait_updates_intensity() -> None:
    personality = NeyraPersonalityCore()
    personality.apply_trait("curiosity", 0.4)
    assert personality.get_trait("curiosity") == 0.4
    personality.apply_trait("curiosity", 0.8)
    # clamped at upper bound
    assert personality.get_trait("curiosity") == 1.0


def test_apply_trait_clamps_lower_bound() -> None:
    personality = NeyraPersonalityCore()
    personality.apply_trait("patience", -0.2)
    assert personality.get_trait("patience") == 0.0


def test_get_reaction_changes_with_trait() -> None:
    personality = NeyraPersonalityCore()
    personality.apply_trait("curiosity", 0.2)
    assert personality.get_reaction("curiosity") == "weak"
    personality.apply_trait("curiosity", 0.2)
    assert personality.get_reaction("curiosity") == "moderate"
    personality.apply_trait("curiosity", 0.5)
    assert personality.get_reaction("curiosity") == "strong"

