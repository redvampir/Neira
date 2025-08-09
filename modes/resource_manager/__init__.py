"""Resource manager mode for viewing, searching and importing resources."""
from .resource_manager import ResourceManagerMode
from help.context_helper import register_walkthrough

# Register walkthrough for this mode
register_walkthrough("resource_manager", lambda: ResourceManagerMode().run())

__all__ = ["ResourceManagerMode"]
