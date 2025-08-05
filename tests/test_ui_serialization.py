"""Tests for simple UI serialization helpers."""

from ui.campaign_interface import CampaignInterface
from ui.virtual_table import VirtualTable
from ui.master_screen import MasterScreen


class _Dummy:
    def serialize(self):  # pragma: no cover - trivial
        return {"foo": "bar"}


def test_campaign_interface_serializes_sections() -> None:
    ci = CampaignInterface()
    ci.add_section("dummy", _Dummy())
    assert ci.render() == {"dummy": {"foo": "bar"}}


def test_virtual_table_serializes_components() -> None:
    table = VirtualTable()
    table.add_component("dummy", _Dummy())
    assert table.render() == {"dummy": {"foo": "bar"}}


def test_master_screen_serializes_tools() -> None:
    screen = MasterScreen()
    screen.add_tool("dummy", _Dummy())
    assert screen.render() == {"dummy": {"foo": "bar"}}
