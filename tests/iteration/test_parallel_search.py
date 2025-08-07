import time

from src.iteration import DeepSearcher, ResourceAwareIterator
from src.iteration.plugin_registry import (
    clear_search_plugins,
    register_search_plugin,
)


class SlowPlugin:
    def __init__(self, delay: float) -> None:
        self.delay = delay

    def search(self, query: str, limit: int = 5):
        time.sleep(self.delay)
        return [
            {
                "source": "slow",
                "reference": "",
                "content": query,
                "priority": 0.1,
            }
        ]


def test_parallel_search_runs_concurrently():
    clear_search_plugins()
    delay = 0.2
    register_search_plugin(SlowPlugin(delay), name="p1")
    register_search_plugin(SlowPlugin(delay), name="p2")
    iterator = ResourceAwareIterator({"cpu": 32})
    searcher = DeepSearcher(resource_iterator=iterator, use_default_plugins=False)

    start = time.perf_counter()
    searcher.search("hello")
    duration = time.perf_counter() - start

    assert duration < delay * 1.5


def test_parallel_search_can_be_disabled():
    clear_search_plugins()
    delay = 0.2
    register_search_plugin(SlowPlugin(delay), name="p1")
    register_search_plugin(SlowPlugin(delay), name="p2")
    iterator = ResourceAwareIterator({"cpu": 2})
    searcher = DeepSearcher(resource_iterator=iterator, use_default_plugins=False)

    start = time.perf_counter()
    searcher.search("hello")
    duration = time.perf_counter() - start

    assert duration >= delay * 2
