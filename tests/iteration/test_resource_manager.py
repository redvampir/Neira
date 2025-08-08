from src.iteration.resource_manager import ResourceManager
from src.iteration import ResourceAwareIterator, IterativeGenerator


def test_low_resource_config() -> None:
    manager = ResourceManager(gpu_memory=2, cpu_cores=2)
    cfg = manager.get_config()
    assert cfg.max_iterations == 2
    assert cfg.parallel is False
    assert cfg.cache["hot_limit"] == 4


def test_high_resource_config() -> None:
    manager = ResourceManager(gpu_memory=16, cpu_cores=16)
    cfg = manager.get_config()
    assert cfg.max_iterations == 8
    assert cfg.parallel is True
    assert cfg.cache["hot_limit"] == 32


def test_integration_with_iterator() -> None:
    manager = ResourceManager(gpu_memory=2, cpu_cores=2)
    iterator = ResourceAwareIterator(resource_manager=manager)
    assert iterator.config.max_iterations == 2
    assert iterator.config.parallel is False


def test_iterative_generator_uses_manager() -> None:
    manager = ResourceManager(gpu_memory=2, cpu_cores=2)
    generator = IterativeGenerator(resource_manager=manager)
    assert generator.iteration_controller.max_iterations == 2
