"""Scheduled knowledge updater."""

from __future__ import annotations

import time
import schedule

from src.search import SearchAPIClient


client = SearchAPIClient()


def update() -> None:
    """Run a sample query and update internal knowledge."""
    client.search_and_update("latest technology news", limit=3)


def main() -> None:  # pragma: no cover - manual script
    schedule.every().hour.do(update)
    while True:
        schedule.run_pending()
        time.sleep(1)


if __name__ == "__main__":  # pragma: no cover - manual script
    main()
