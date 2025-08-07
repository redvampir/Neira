from src.iteration import FeedbackLearner
from src.memory import CharacterMemory, WorldMemory, StyleMemory


def test_feedback_updates_memories(tmp_path):
    char_mem = CharacterMemory(tmp_path / "chars.json")
    world_mem = WorldMemory(tmp_path / "world.json")
    style_mem = StyleMemory(tmp_path / "style.json")
    learner = FeedbackLearner(char_mem, world_mem, style_mem)

    feedback = [
        {"type": "character", "confirmed": True, "data": {"name": "Bob", "appearance": "tall"}},
        {
            "type": "world",
            "confirmed": True,
            "data": {
                "name": "Earth",
                "info": {"rules": [{"category": "physics", "description": "gravity"}]},
            },
        },
        {
            "type": "style",
            "confirmed": True,
            "data": {
                "user_id": "u1",
                "author": "Bob",
                "example": "Hello world",
                "description": "friendly",
            },
        },
        {"type": "character", "confirmed": False, "data": {"name": "Ignored"}},
    ]

    learner.apply(feedback)

    assert char_mem.get("Bob").appearance == "tall"
    world_rules = world_mem.get("Earth")["rules"]
    assert world_rules[0]["category"] == "physics"
    style = style_mem.get_style("u1", "Bob")
    assert "Hello world" in style.examples
    assert char_mem.get("Ignored") is None
