"""Tests for the StoryWeaver action."""

import sys
from pathlib import Path

# Ensure project root is on the path for src layout
sys.path.append(str(Path(__file__).resolve().parents[2]))

from src.action.story_weaver import StoryWeaver


def test_weaves_with_transitions() -> None:
    weaver = StoryWeaver()
    scenes = [
        "Alice enters the room.",
        "She looks around.",
        "Bob waves.",
    ]

    narrative = weaver.weave(scenes)
    assert (
        narrative == "Alice enters the room. Then, she looks around. Next, Bob waves."
    )


def test_resolves_conflicting_states() -> None:
    weaver = StoryWeaver()
    scenes = [
        "Alice is happy.",
        "Alice is sad.",
        "Bob smiles.",
    ]

    narrative = weaver.weave(scenes)
    assert narrative == "Alice is happy. However, Alice is sad. Next, Bob smiles."


def test_removes_duplicate_scenes() -> None:
    weaver = StoryWeaver()
    scenes = [
        "Bob laughs.",
        "Bob laughs.",
        "Bob cries.",
    ]

    narrative = weaver.weave(scenes)
    assert narrative == "Bob laughs. Then, Bob cries."
