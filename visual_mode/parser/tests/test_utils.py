from __future__ import annotations

from visual_mode.parser.utils import (
    lookup_localization,
    offset_to_position,
    position_to_offset,
)


def test_lookup_localization_returns_value_or_key() -> None:
    mapping = {"hello": "Привет"}
    assert lookup_localization("hello", mapping) == "Привет"
    assert lookup_localization("bye", mapping) == "bye"


def test_position_roundtrip() -> None:
    text = "one\ntwo\n"
    offset = position_to_offset(text, 2, 2)
    assert offset_to_position(text, offset) == (2, 2)
