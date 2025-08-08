from src.neurons import NeuronFactory
from src.neurons.partners import run_partners
from src.neurons.network import NeuronNetwork


def setup_function() -> None:
    NeuronFactory._registry.clear()  # type: ignore[attr-defined]
    NeuronFactory.load_plugins()


def test_partner_neurons_registration() -> None:
    creative = NeuronFactory.create("creative_partner", id="c")
    logical = NeuronFactory.create("logical_partner", id="l")
    assert creative.process("hello").startswith("[Creative]")
    assert logical.process("world").startswith("[Logical]")


def test_partner_orchestrator_and_network() -> None:
    result = run_partners("hello")
    assert result == "[Logical] [Creative] hello"
    network = NeuronNetwork()
    assert network.process("partner: hi") == "[Logical] [Creative] hi"
    assert network.process("other") == "processed: other"
