from pathlib import Path
import sys
import types

# Stub external dependencies used by configuration
yaml = types.ModuleType("yaml")
yaml.safe_load = lambda s: {}
sys.modules.setdefault("yaml", yaml)

dotenv = types.ModuleType("dotenv")
dotenv.load_dotenv = lambda *a, **k: None
sys.modules.setdefault("dotenv", dotenv)

sys.path.append(str(Path(__file__).resolve().parents[2]))

from ui.command_palette import CommandPalette, register_command
from src.plugins import PluginManager


def test_search_by_name_and_argument():
    def sample_action(foo, bar):
        return foo, bar

    register_command("sample", sample_action)
    palette = CommandPalette()

    # Search by command name
    names = [name for name, _ in palette.search("sam")]
    assert "sample" in names

    # Search by argument name
    names = [name for name, _ in palette.search("bar")]
    assert "sample" in names


def test_plugin_can_register_command(tmp_path):
    plugin_dir = tmp_path / "plugins"
    plugin_dir.mkdir()
    plugin_file = plugin_dir / "cmd_plugin.py"
    plugin_file.write_text(
        "from src.plugins import Plugin\n"
        "from ui.command_palette import register_command\n"
        "class CmdPlugin(Plugin):\n    pass\n"
        "def greet(name):\n    return f'hi {name}'\n"
        "register_command('greet', greet)\n"
    )

    PluginManager(plugin_dir)
    palette = CommandPalette()
    names = [name for name, _ in palette.search("greet")]
    assert "greet" in names


def test_hotkey_registered():
    palette = CommandPalette()
    keys = [k for b in palette.key_bindings.bindings for k in b.keys]
    assert "c-S-p" in keys or "c-P" in keys
