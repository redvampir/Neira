"""Minimal Language Server Protocol (LSP) client.

This module provides a small utility wrapper around a language server
process.  It communicates using the LSP specification over the
``stdio`` transport.  The client is intentionally lightweight and does
not depend on external packages so it can be easily embedded in simple
scripts or tests.
"""

from __future__ import annotations

import json
import os
import subprocess
import threading
from dataclasses import dataclass, field
from pathlib import Path
from queue import Empty, Queue
from typing import Any, Dict, List, Optional


@dataclass
class LSPClient:
    """Basic LSP client speaking the ``stdio`` transport."""

    server_command: List[str] = field(default_factory=lambda: ["pylsp"])
    root_uri: str = field(default_factory=lambda: Path.cwd().as_uri())

    _process: Optional[subprocess.Popen] = field(init=False, default=None)
    _recv_queue: "Queue[str]" = field(init=False, default_factory=Queue)
    _reader_thread: Optional[threading.Thread] = field(init=False, default=None)
    _id: int = field(init=False, default=0)

    # ------------------------------------------------------------------
    def start(self) -> None:
        """Start the language server process and send ``initialize``."""

        if self._process is not None:
            return

        self._process = subprocess.Popen(
            self.server_command,
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.DEVNULL,
            text=True,
        )

        self._reader_thread = threading.Thread(target=self._reader, daemon=True)
        self._reader_thread.start()

        self._send(
            {
                "jsonrpc": "2.0",
                "id": self._next_id(),
                "method": "initialize",
                "params": {
                    "processId": os.getpid(),
                    "rootUri": self.root_uri,
                    "capabilities": {},
                },
            }
        )

    # ------------------------------------------------------------------
    def _reader(self) -> None:
        assert self._process and self._process.stdout
        while True:
            line = self._process.stdout.readline()
            if not line:
                break
            if line.startswith("Content-Length:"):
                length = int(line.split(":")[1].strip())
                # skip header end line
                self._process.stdout.readline()
                content = self._process.stdout.read(length)
                self._recv_queue.put(content)

    # ------------------------------------------------------------------
    def _send(self, payload: Dict[str, Any]) -> None:
        if not self._process or not self._process.stdin:
            raise RuntimeError("LSP server not started")
        body = json.dumps(payload)
        message = f"Content-Length: {len(body)}\r\n\r\n{body}"
        self._process.stdin.write(message)
        self._process.stdin.flush()

    # ------------------------------------------------------------------
    def _next_id(self) -> int:
        self._id += 1
        return self._id

    # ------------------------------------------------------------------
    def completion(self, uri: str, line: int, character: int) -> Any:
        request_id = self._next_id()
        self._send(
            {
                "jsonrpc": "2.0",
                "id": request_id,
                "method": "textDocument/completion",
                "params": {
                    "textDocument": {"uri": uri},
                    "position": {"line": line, "character": character},
                },
            }
        )
        return self._await_response(request_id)

    # ------------------------------------------------------------------
    def _await_response(self, request_id: int, timeout: float = 5) -> Any:
        try:
            while True:
                message = self._recv_queue.get(timeout=timeout)
                data = json.loads(message)
                if data.get("id") == request_id:
                    return data.get("result")
        except Empty:
            return None

    # ------------------------------------------------------------------
    def shutdown(self) -> None:
        try:
            if self._process:
                self._send({"jsonrpc": "2.0", "id": self._next_id(), "method": "shutdown"})
                self._send({"jsonrpc": "2.0", "method": "exit"})
        finally:
            if self._process:
                self._process.terminate()
                self._process = None
