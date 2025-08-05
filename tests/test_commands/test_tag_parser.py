"""Tests for command tag parser."""

import sys
from pathlib import Path

# Ensure project root is on the import path for src layout
sys.path.append(str(Path(__file__).resolve().parents[2]))

from src.commands.tag_parser import CommandTagParser


def test_parser_routes_known_commands() -> None:
    parser = CommandTagParser()
    text = "@Кампания: создать@ [Настроить НРИ-режим]"
    executed = parser.parse_and_execute(text)
    assert executed == ["create_campaign", "setup_nri_mode"]


def test_parser_is_extensible() -> None:
    parser = CommandTagParser()

    parser.register("dummy", r"@Тест@", lambda _: "dummy")
    assert parser.parse_and_execute("@Тест@") == ["dummy"]
