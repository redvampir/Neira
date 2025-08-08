import gc
import sys
import weakref

from src.core import adaptive_loader


def test_enable_and_disable_releases_resources(tmp_path):
    module_name = "dummy_module"
    module_code = (
        "class Big:\n"
        "    def __init__(self):\n"
        "        self.data = [0] * 100000\n"
        "DATA = Big()\n"
    )
    module_file = tmp_path / f"{module_name}.py"
    module_file.write_text(module_code, encoding="utf-8")
    sys.path.insert(0, str(tmp_path))

    try:
        rm = adaptive_loader.resource_manager
        before_cpu = rm.available_cpu

        mod = adaptive_loader.enable(module_name)
        assert module_name in sys.modules
        assert rm.available_cpu == before_cpu - 1

        ref = weakref.ref(mod.DATA)

        adaptive_loader.disable(module_name)
        del mod
        gc.collect()

        assert ref() is None
        assert module_name not in sys.modules
        assert rm.available_cpu == before_cpu
    finally:
        sys.path.remove(str(tmp_path))


def test_determine_active_components_respects_profile(monkeypatch):
    def fake_update() -> tuple[float, float]:
        return 99.0, 99.0

    monkeypatch.setattr(adaptive_loader.resource_manager, "update_usage", fake_update)
    result = adaptive_loader.determine_active_components("medium", ["a", "b"])
    assert result == []

