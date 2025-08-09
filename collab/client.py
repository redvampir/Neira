"""WebSocket client for collaborative editors.

The client keeps track of cursor positions of other users. Editors can
connect to a shared :class:`CollabServer` and update ``remote_cursors`` to
render other users' cursors locally.
"""

from __future__ import annotations

import asyncio
import json
from typing import Dict, Optional

try:  # pragma: no cover - optional dependency
    import websockets
except Exception:  # pragma: no cover - handled during runtime
    websockets = None  # type: ignore


class CollabClient:
    """Minimal collaborative editing client.

    Parameters
    ----------
    uri:
        WebSocket server URI.
    user:
        Identifier of the current user.
    """

    def __init__(self, uri: str, user: str) -> None:
        self.uri = uri
        self.user = user
        self.websocket: Optional[websockets.WebSocketClientProtocol] = None
        self.remote_cursors: Dict[str, int] = {}
        self._listener: Optional[asyncio.Task] = None

    async def connect(self) -> None:
        """Connect to the collaborative server and start receiving events."""

        if websockets is None:
            raise RuntimeError("websockets library is required for CollabClient")
        self.websocket = await websockets.connect(self.uri)
        self._listener = asyncio.create_task(self._receiver())

    async def _receiver(self) -> None:
        assert self.websocket is not None
        async for message in self.websocket:
            data = json.loads(message)
            if (
                data.get("type") == "cursor"
                and data.get("user") != self.user
            ):
                self.remote_cursors[data["user"]] = int(data.get("position", 0))

    async def send_cursor(self, position: int) -> None:
        """Broadcast the user's current cursor ``position``."""

        if self.websocket is None:
            raise RuntimeError("Client is not connected")
        payload = {"type": "cursor", "user": self.user, "position": position}
        await self.websocket.send(json.dumps(payload))

    async def send_change(self, file_path: str, content: str) -> None:
        """Send a file change to the server.

        The server is responsible for committing the change and rebroadcasting
        it to other clients.
        """

        if self.websocket is None:
            raise RuntimeError("Client is not connected")
        payload = {
            "type": "change",
            "user": self.user,
            "file": file_path,
            "content": content,
        }
        await self.websocket.send(json.dumps(payload))

    async def close(self) -> None:
        """Close the connection to the server."""

        if self.websocket is not None:
            await self.websocket.close()
            self.websocket = None
        if self._listener is not None:
            self._listener.cancel()
            self._listener = None
