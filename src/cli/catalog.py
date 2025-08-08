"""Command line helper for interacting with :class:`IdeaCatalog`."""

from __future__ import annotations

import argparse
from typing import Sequence

from src.memory.idea_catalog import IdeaCatalog


def main(argv: Sequence[str] | None = None) -> None:
    parser = argparse.ArgumentParser(description="View or modify idea catalog")
    sub = parser.add_subparsers(dest="cmd")

    sub.add_parser("list", help="List catalog entries")

    add_p = sub.add_parser("add", help="Add new entry")
    add_p.add_argument("key")
    add_p.add_argument("text")

    show_p = sub.add_parser("show", help="Display entry")
    show_p.add_argument("key")

    del_p = sub.add_parser("delete", help="Remove entry")
    del_p.add_argument("key")

    args = parser.parse_args(argv)
    catalog = IdeaCatalog()

    if args.cmd == "list":
        data = catalog.get()
        for key, val in data.items():
            print(f"{key}: {val}")
    elif args.cmd == "add":
        catalog.add(args.key, args.text)
        catalog.save()
    elif args.cmd == "show":
        value = catalog.get(args.key)
        if value is not None:
            print(value)
    elif args.cmd == "delete":
        catalog.delete(args.key)
        catalog.save()
    else:  # pragma: no cover - argparse already prints help
        parser.print_help()


if __name__ == "__main__":  # pragma: no cover - CLI entry point
    main()
