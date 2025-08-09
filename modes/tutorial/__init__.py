"""Tutorial mode with simple lesson scenes."""

from .tutorial import Tutorial
from help.context_helper import register_walkthrough

# Register tutorial walkthrough
register_walkthrough("tutorial", lambda: Tutorial().run())

__all__ = ["Tutorial"]
