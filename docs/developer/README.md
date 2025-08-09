# Developer Guide

This guide describes the expected interfaces and the plugin architecture.

## Interface standards

Modes and other components follow a minimal contract: they expose `start()`
and `stop()` methods and receive a context object that provides logging and
access to shared services. External code should interact with modes only
through this interface.

## Plugin structure

Plugins extend the `Plugin` base class and override hook methods.

```python
from src.plugins import Plugin

class ExamplePlugin(Plugin):
    def on_draft(self, draft, context):
        ...

    def on_gap_analysis(self, draft, gaps):
        ...

    def on_finalize(self, response):
        ...
```

Plugins are discovered by the `PluginManager` and can subscribe to any subset
of hooks. Each plugin should be self-contained and avoid side effects outside
the provided context.
