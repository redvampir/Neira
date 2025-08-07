from src.iteration import DeepSearcher
from src.iteration import DeepSearcher
from src.iteration.plugin_registry import (
    clear_search_plugins,
    get_search_plugins,
    register_search_plugin,
)


class DummyPlugin:
    """Simple plugin returning the query as content."""

    def search(self, query: str, limit: int = 5):
        return [
            {
                "source": "dummy",
                "reference": "",
                "content": query,
                "priority": 0.42,
            }
        ]


def test_register_and_use_plugin():
    clear_search_plugins()
    register_search_plugin(DummyPlugin())
    assert len(get_search_plugins()) == 1

    searcher = DeepSearcher(use_default_plugins=False)
    results = searcher.search("hello")

    assert results == [
        {
            "source": "dummy",
            "reference": "",
            "content": "hello",
            "priority": 0.42,
        }
    ]
