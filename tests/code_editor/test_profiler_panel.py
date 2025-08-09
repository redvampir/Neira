import sys
from pathlib import Path

sys.path.append(str(Path(__file__).resolve().parents[2]))

from code_editor.profiler_panel import ProfilerPanel


def test_profile_and_report():
    panel = ProfilerPanel()

    def dummy():
        return 21

    result = panel.profile("dummy", dummy)
    assert result == 21

    report = panel.report()
    assert "dummy" in report

    suggestions = panel.suggestions()
    assert isinstance(suggestions, list)
