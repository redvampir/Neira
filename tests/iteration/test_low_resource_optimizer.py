from src.iteration import LowResourceOptimizer, ResourceAwareIterator


def test_low_memory_suggestion() -> None:
    optimizer = LowResourceOptimizer({"cpu": 2})
    config = optimizer.suggest()
    assert config["parallel"] is False
    assert config["cache"] == {"hot_limit": 4, "warm_limit": 16}


def test_high_memory_suggestion() -> None:
    optimizer = LowResourceOptimizer({"cpu": 32})
    config = optimizer.suggest()
    assert config["parallel"] is True
    assert config["cache"] == {"hot_limit": 32, "warm_limit": 128}


def test_integration_with_iterator() -> None:
    iterator = ResourceAwareIterator({"cpu": 2})
    assert iterator.config["parallel"] is False
    assert iterator.config["cache"]["hot_limit"] == 4

