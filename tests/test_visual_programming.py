import types
from visual_programming.debug.step_panel import StepDebugger
from visual_programming.watch import WatchNode
from visual_programming.code_generator import save_generated_code, GENERATED_DIR


def test_step_debugger_and_watch(tmp_path):
    counter = {'x': 0}

    def inc():
        counter['x'] += 1

    dbg = StepDebugger()
    dbg.add_step(inc)
    dbg.add_step(inc)
    watch = dbg.add_watch('x', lambda: counter['x'])

    dbg.step()
    assert watch.value == 1
    dbg.step()
    assert watch.value == 2
    dbg.reset()
    assert watch.value == 2  # resetting does not change counter


def test_save_generated_code(tmp_path):
    path = save_generated_code('sample', 'print("hello")')
    assert path.exists()
    assert path.parent == GENERATED_DIR
    with open(path) as f:
        assert 'print("hello")' in f.read()
