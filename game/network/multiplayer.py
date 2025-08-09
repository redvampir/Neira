"""Simple WebSocket based multiplayer utilities."""

from __future__ import annotations

import asyncio
from typing import Set, Callable, Awaitable, Optional

try:
    import websockets
    from websockets.server import WebSocketServerProtocol
except ImportError:  # pragma: no cover - handled during runtime
    websockets = None  # type: ignore
    WebSocketServerProtocol = object  # type: ignore


class MultiplayerServer:
    """Tiny WebSocket broadcast server.

    The server relays every received message to all connected clients.
    It is intentionally lightweight so it can be embedded inside tests or
    small games without external dependencies beyond ``websockets``.
    """

    def __init__(self, host: str = "localhost", port: int = 8765) -> None:
        self.host = host
        self.port = port
        self.clients: Set[WebSocketServerProtocol] = set()
        self._server: Optional[asyncio.base_events.Server] = None

    async def _handler(self, websocket: WebSocketServerProtocol, path: str) -> None:
        self.clients.add(websocket)
        try:
            async for message in websocket:
                await self.broadcast(message, sender=websocket)
        finally:
            self.clients.discard(websocket)

    async def broadcast(
        self, message: str, sender: Optional[WebSocketServerProtocol] = None
    ) -> None:
        """Send ``message`` to all connected clients except ``sender``."""

        if not self.clients:
            return
        await asyncio.gather(
            *[client.send(message) for client in self.clients if client is not sender]
        )

    async def start(self) -> None:
        if websockets is None:
            raise RuntimeError("websockets library is required for MultiplayerServer")
        self._server = await websockets.serve(self._handler, self.host, self.port)

    async def stop(self) -> None:
        if self._server is not None:
            self._server.close()
            await self._server.wait_closed()
            self._server = None


class MultiplayerClient:
    """Minimal WebSocket client for multiplayer games."""

    def __init__(self, uri: str) -> None:
        self.uri = uri
        self.websocket: Optional[websockets.WebSocketClientProtocol] = None

    async def connect(self) -> None:
        if websockets is None:
            raise RuntimeError("websockets library is required for MultiplayerClient")
        self.websocket = await websockets.connect(self.uri)

    async def send(self, message: str) -> None:
        if self.websocket is None:
            raise RuntimeError("Client is not connected")
        await self.websocket.send(message)

    async def receive(self) -> str:
        if self.websocket is None:
            raise RuntimeError("Client is not connected")
        return await self.websocket.recv()

    async def close(self) -> None:
        if self.websocket is not None:
            await self.websocket.close()
            self.websocket = None
