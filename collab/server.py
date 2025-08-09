"""WebSocket based collaboration server.

The server relays cursor positions and file changes between connected clients.
File changes are automatically committed to the Git repository and every
message is appended to a session log inside ``collab/history``.
"""

from __future__ import annotations

import asyncio
import datetime as _dt
import json
import subprocess
from pathlib import Path
from typing import Dict, Optional

try:  # pragma: no cover - optional dependency
    import websockets
    from websockets.server import WebSocketServerProtocol
except Exception:  # pragma: no cover - handled during runtime
    websockets = None  # type: ignore
    WebSocketServerProtocol = object  # type: ignore


class CollabServer:
    """Collaboration WebSocket server.

    Parameters
    ----------
    host, port:
        Address on which to listen for connections.
    repo_root:
        Root directory of the Git repository whose files are being edited.
    """

    def __init__(self, host: str = "localhost", port: int = 8765, repo_root: Optional[Path] = None) -> None:
        self.host = host
        self.port = port
        self.repo_root = Path(repo_root or Path.cwd())
        self.clients: Dict[str, WebSocketServerProtocol] = {}
        self._server: Optional[asyncio.base_events.Server] = None
        history_root = Path(__file__).with_name("history")
        history_root.mkdir(parents=True, exist_ok=True)
        ts = _dt.datetime.utcnow().strftime("%Y%m%d%H%M%S")
        self.log_file = history_root / f"session-{ts}.log"

    async def _handler(self, websocket: WebSocketServerProtocol, path: str) -> None:
        user: Optional[str] = None
        try:
            async for raw in websocket:
                data = json.loads(raw)
                user = data.get("user")
                msg_type = data.get("type")
                if msg_type == "cursor":
                    if user:
                        self.clients[user] = websocket
                    await self.broadcast(raw, sender=websocket)
                    self._log(data)
                elif msg_type == "change":
                    commit = self._apply_change(data)
                    await self.broadcast(raw, sender=websocket)
                    self._log(data, commit)
        finally:
            if user and user in self.clients:
                del self.clients[user]

    async def broadcast(self, message: str, sender: Optional[WebSocketServerProtocol] = None) -> None:
        """Send ``message`` to all clients except ``sender``."""

        if not self.clients:
            return
        await asyncio.gather(
            *[
                ws.send(message)
                for ws in self.clients.values()
                if ws is not sender
            ]
        )

    def _apply_change(self, data: Dict) -> Optional[str]:
        file_path = data.get("file")
        content = data.get("content")
        if not file_path or content is None:
            return None
        path = self.repo_root / file_path
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(content, encoding="utf-8")
        subprocess.run(["git", "add", file_path], cwd=self.repo_root, check=False)
        subprocess.run(
            ["git", "commit", "-m", f"Collab edit by {data.get('user', 'unknown')}"],
            cwd=self.repo_root,
            check=False,
        )
        return self._current_commit()

    def _current_commit(self) -> Optional[str]:
        result = subprocess.run(
            ["git", "rev-parse", "HEAD"],
            cwd=self.repo_root,
            capture_output=True,
            text=True,
            check=False,
        )
        return result.stdout.strip() or None

    def _log(self, data: Dict, commit: Optional[str] = None) -> None:
        record = {"timestamp": _dt.datetime.utcnow().isoformat(), **data}
        if commit:
            record["commit"] = commit
        with self.log_file.open("a", encoding="utf-8") as fh:
            fh.write(json.dumps(record) + "\n")

    async def start(self) -> None:
        if websockets is None:
            raise RuntimeError("websockets library is required for CollabServer")
        self._server = await websockets.serve(self._handler, self.host, self.port)

    async def stop(self) -> None:
        if self._server is not None:
            self._server.close()
            await self._server.wait_closed()
            self._server = None
