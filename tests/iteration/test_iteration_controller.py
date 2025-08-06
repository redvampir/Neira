from src.iteration.iteration_controller import IterationController


def test_assess_quality_counts_placeholders():
    controller = IterationController()
    text = "Start ___ middle ___ end"
    assert controller.assess_quality(text) == 2


def test_iteration_stops_on_quality_and_limit():
    controller = IterationController(max_iterations=2, max_critical_spaces=1)

    # First call - quality insufficient and limit not reached
    assert controller.should_iterate("___ ___") is True
    # Second call - still insufficient but now at limit after this
    assert controller.should_iterate("___ ___") is True
    # Third call - limit reached so no further iterations
    assert controller.should_iterate("___ ___") is False

    # Quality check - when gaps under threshold it stops immediately
    controller = IterationController(max_iterations=5, max_critical_spaces=2)
    assert controller.should_iterate("only one ___") is False
