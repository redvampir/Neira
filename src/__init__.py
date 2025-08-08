"""Top-level package for Neyra."""

from neira_rust import ping as rust_ping


def ping() -> str:
    """Return the result of the Rust `ping` function."""
    return rust_ping()


# Simple check to ensure the Rust extension is linked correctly
assert ping() == "pong"
