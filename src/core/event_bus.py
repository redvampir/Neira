from __future__ import annotations

"""Simple asynchronous event bus with queue based dispatch."""

from dataclasses import dataclass
import asyncio
import inspect
import threading
import time
from typing import Any, Awaitable, Callable, Dict, List, TypeVar, Generic

from src.core.config import get_logger

logger = get_logger(__name__)


PayloadT = TypeVar("PayloadT")


@dataclass(slots=True)
class Event(Generic[PayloadT]):
    """Typed event container."""

    type: str
    payload: PayloadT


Handler = Callable[[Event[Any]], Awaitable[None] | None]


class EventBus:
    """Minimal asynchronous event bus with an internal queue.

    The bus runs a private asyncio loop in a background thread so that events
    can be published from synchronous code. Handlers may be regular or async
    callables and are processed in FIFO order.
    """

    def __init__(self) -> None:
        self._subscribers: Dict[str, List[Handler]] = {}
        self._loop = asyncio.new_event_loop()
        self._queue: asyncio.Queue[Event[Any]] | None = None
        self._thread = threading.Thread(target=self._run_loop, daemon=True)
        self._thread.start()
        # initialise queue and dispatcher once loop is running
        def _init() -> None:
            assert self._loop is not None
            self._queue = asyncio.Queue()
            self._loop.create_task(self._dispatcher())

        self._loop.call_soon_threadsafe(_init)

    # ------------------------------------------------------------------
    def subscribe(self, event_type: str, handler: Handler) -> None:
        """Register a handler for ``event_type`` events."""

        self._subscribers.setdefault(event_type, []).append(handler)

    # ------------------------------------------------------------------
    def publish(self, event: Event[Any]) -> None:
        """Publish an event by placing it into the internal queue."""

        # wait until queue is initialised by background thread
        while self._queue is None:
            time.sleep(0.01)
        asyncio.run_coroutine_threadsafe(self._queue.put(event), self._loop)

    # ------------------------------------------------------------------
    def _run_loop(self) -> None:
        asyncio.set_event_loop(self._loop)
        self._loop.run_forever()

    async def _dispatcher(self) -> None:
        assert self._queue is not None
        while True:
            event = await self._queue.get()
            handlers = list(self._subscribers.get(event.type, []))
            for handler in handlers:
                try:
                    result = handler(event)
                    if inspect.isawaitable(result):
                        await result
                except Exception:  # pragma: no cover - log and continue
                    logger.exception("Error handling event %s", event.type)

__all__ = ["Event", "EventBus"]
