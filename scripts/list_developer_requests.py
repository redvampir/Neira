"""Summarize outstanding developer requests."""

from __future__ import annotations

from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.append(str(ROOT))


def main() -> None:
    """Print the contents of ``logs/developer_requests.md``."""
    path = ROOT / "logs" / "developer_requests.md"
    if not path.exists():
        print("No developer requests found.")
        return

    lines = [
        line.strip()
        for line in path.read_text(encoding="utf-8").splitlines()
        if line.strip().startswith(("-", "*"))
    ]
    if not lines:
        print("No developer requests found.")
        return

    print("Outstanding developer requests:")
    for line in lines:
        if line[0] in {"-", "*"}:
            line = line[1:].strip()
        print(f"- {line}")


if __name__ == "__main__":  # pragma: no cover - helper script
    main()
