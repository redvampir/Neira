from src.plugins import Plugin


class ExamplePlugin(Plugin):
    """Small plugin used in tests to record hook execution."""

    def __init__(self) -> None:
        self.events: list[tuple[str, object]] = []

    def on_draft(self, draft, context):  # pragma: no cover - exercised in tests
        self.events.append(("draft", draft))

    def on_gap_analysis(self, draft, gaps):  # pragma: no cover - exercised in tests
        self.events.append(("gap", len(gaps)))

    def on_finalize(self, response):  # pragma: no cover - exercised in tests
        self.events.append(("final", response))
