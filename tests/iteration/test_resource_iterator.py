from src.iteration import ResourceAwareIterator


def test_plan_basic() -> None:
    iterator = ResourceAwareIterator({"gpu": 4, "cpu": 8, "time": 120})
    plan = iterator.plan({"gpu": 2, "cpu": 1, "time": 30})
    assert plan == [0, 1]


def test_plan_no_gpu() -> None:
    iterator = ResourceAwareIterator({"cpu": 8, "time": 120})
    plan = iterator.plan({"gpu": 2, "cpu": 1, "time": 60})
    assert plan == []


def test_plan_ignores_unused_resources() -> None:
    iterator = ResourceAwareIterator({"gpu": 16, "cpu": 8, "time": 120})
    plan = iterator.plan({"cpu": 2, "time": 30})
    assert plan == [0, 1, 2, 3]
