from __future__ import annotations

from src.neurons import NeuronFactory
from src.memory import CharacterMemory
from src.models import Character


def test_character_neuron_plugin_registration_and_voice(tmp_path) -> None:
    memory = CharacterMemory(tmp_path / "chars.json")
    memory.add(
        Character(
            name="Alice", personality_traits=["brave"], appearance="steampunk"
        )
    )

    NeuronFactory._registry.clear()  # type: ignore[attr-defined]
    NeuronFactory.load_plugins()

    neuron = NeuronFactory.create(
        "character", id="c1", name="Alice", memory=memory
    )
    response = neuron.process("Hello there")
    assert "Alice" in response
    assert "brave" in response
