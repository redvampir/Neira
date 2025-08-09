import sys
from pathlib import Path

sys.path.append(str(Path(__file__).resolve().parents[2]))
import numpy as np

from image_editor.layers import (
    Layer,
    generate_ai_layer,
    invert_effect,
)
from image_editor.svg import export_svg, import_svg


def test_mask_and_effects():
    data = np.ones((2, 2, 3), dtype=np.uint8) * 100
    mask = np.array([[1, 0], [0, 1]], dtype=np.float32)
    layer = Layer(name="test", content=data, mask=mask, effects=[invert_effect])
    rendered = layer.render()
    expected = np.array(
        [
            [[155, 155, 155], [255, 255, 255]],
            [[255, 255, 255], [155, 155, 155]],
        ],
        dtype=np.uint8,
    )
    assert np.array_equal(rendered, expected)


def test_svg_import_export(tmp_path):
    vec = ['<rect x="0" y="0" width="10" height="10" />']
    file_path = tmp_path / "shape.svg"
    export_svg(vec, str(file_path))
    loaded = import_svg(str(file_path))
    assert vec == loaded


def test_ai_layer_generation():
    layer = generate_ai_layer("test", size=(8, 8))
    assert layer.content.shape == (8, 8, 3)
