import os
import sys

sys.path.append(os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..")))

from characters.compact import CompactCharacter


def test_serialization_roundtrip():
    original = CompactCharacter(
        name="Hero",
        core_traits=["brave", "cunning"],
        story_moments=["battle_of_hill"],
    )

    json_data = original.to_json()
    recreated = CompactCharacter.from_json(json_data)

    assert recreated == original


def test_extract_new_traits_with_limit():
    char = CompactCharacter(
        name="LimitTester",
        core_traits=["a", "b", "c", "d", "e"],
    )
    context = "f g h"
    patterns = {"f": "f", "g": "g", "h": "h"}

    updated = char._extract_new_traits(context, patterns)

    assert updated == ["d", "e", "f", "g", "h"]
    assert len(updated) == char.MAX_CORE_TRAITS


def test_expand_from_templates():
    char = CompactCharacter(
        name="TemplateTester",
        core_traits=["brave"],
        story_moments=["battle"],
    )

    expanded = char._expand_from_templates(
        trait_templates={"brave": "Shows bravery"},
        moment_templates={"battle": "Fought a great battle"},
    )

    assert expanded["core_traits"] == ["Shows bravery"]
    assert expanded["story_moments"] == ["Fought a great battle"]
