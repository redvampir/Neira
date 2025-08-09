from pathlib import Path
from typing import Union


GENERATED_DIR = Path(__file__).resolve().parent / "generated"
GENERATED_DIR.mkdir(parents=True, exist_ok=True)


def save_generated_code(name: str, code: Union[str, bytes]) -> Path:
    """Save generated source code into the generated directory.

    Parameters
    ----------
    name: str
        Base filename without extension.
    code: Union[str, bytes]
        Source code to write. ``bytes`` are written in binary mode.

    Returns
    -------
    Path
        Path to the written file.
    """

    path = GENERATED_DIR / f"{name}.py"
    mode = "wb" if isinstance(code, bytes) else "w"
    with open(path, mode) as f:
        f.write(code)
    return path
