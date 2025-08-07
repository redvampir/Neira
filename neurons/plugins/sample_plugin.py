"""Sample neuron plugin for Neira.

This module demonstrates how to extend Neira with custom neuron types.
Plugins placed in ``neurons/plugins`` are automatically discovered and
registered by :class:`src.neurons.factory.NeuronFactory`.
"""

from src.neurons import Neuron


class SamplePluginNeuron(Neuron):
    """Simple neuron example used for plugin documentation."""

    type = "sample_plugin"

    def process(self, *args, **kwargs):
        """No-op process method for demonstration purposes."""
        return None
