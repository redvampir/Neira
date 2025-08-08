from pathlib import Path
import sys
import logging

sys.path.append(str(Path(__file__).resolve().parents[2]))

from src.plugins import PluginManager


def test_plugin_manager_continues_on_plugin_error(tmp_path, caplog):
    plugin_dir = tmp_path

    (plugin_dir / "a_fail.py").write_text(
        "from src.plugins import Plugin\n"
        "class FailPlugin(Plugin):\n"
        "    def on_draft(self, draft, context):\n"
        "        raise RuntimeError('boom')\n",
        encoding="utf-8",
    )

    (plugin_dir / "b_ok.py").write_text(
        "from src.plugins import Plugin\n"
        "class OkPlugin(Plugin):\n"
        "    def on_draft(self, draft, context):\n"
        "        context.append('ok')\n",
        encoding="utf-8",
    )

    manager = PluginManager(plugin_dir)
    manager.plugins = sorted(manager.plugins, key=lambda p: p.__class__.__name__)
    context = []
    with caplog.at_level(logging.WARNING):
        manager.on_draft("draft", context)

    assert context == ["ok"]
    assert any("FailPlugin" in r.getMessage() for r in caplog.records)
