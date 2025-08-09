from __future__ import annotations

"""Utility audio engine for playing sound effects in game sessions.

The :class:`SoundEngine` class provides a very small wrapper that can load
pre-recorded assets from ``assets/audio`` and optionally fall back to a
user supplied asynchronous generative service.  Loaded assets are cached in
memory to avoid repeated disk access.  The main entry point is the
:meth:`play` coroutine which accepts basic playback options like volume,
number of loops and output channel.
"""

import asyncio
from pathlib import Path
from typing import Awaitable, Callable, Dict, Optional

from src.core.config import get_logger, load_config_file, settings


# Type alias for the generative service callback
Generator = Callable[[str, Dict[str, object]], Awaitable[bytes]]


class SoundEngine:
    """Simple sound effect manager.

    Parameters
    ----------
    config_name:
        Name of the configuration file inside the :mod:`config` directory.
        Defaults to ``"audio.yaml"``.
    generative_service:
        Optional coroutine used to generate audio when assets are missing or
        when running in ``generation`` mode.  It must accept the ``effect_id``
        and a mapping of arbitrary options and return raw audio bytes.
    """

    def __init__(
        self,
        config_name: str = "audio.yaml",
        generative_service: Optional[Generator] = None,
    ) -> None:
        self.logger = get_logger(__name__)
        self.config: Dict[str, object] = {}
        try:
            self.config = load_config_file(config_name) or {}
        except FileNotFoundError:
            self.logger.warning("Audio config %s not found, using defaults", config_name)
        self.mode: str = str(self.config.get("mode", "assets"))
        asset_dir = self.config.get("asset_dir")
        base = settings.paths.base_dir
        self.asset_dir = (
            Path(asset_dir) if asset_dir else base / "assets" / "audio"
        )
        self.generative_service: Generator = generative_service or self._dummy_generate
        self.cache: Dict[str, bytes] = {}
        self.channels: Dict[str, list[tuple[str, float, int]]] = {}

    # ------------------------------------------------------------------
    async def _dummy_generate(self, effect_id: str, options: Dict[str, object]) -> bytes:
        """Fallback generator used when no service is configured."""

        self.logger.debug("Generating placeholder sound for %s", effect_id)
        await asyncio.sleep(0)  # simulate async call
        return b""  # placeholder empty sound

    # ------------------------------------------------------------------
    def _asset_path(self, effect_id: str) -> Optional[Path]:
        """Return the path to a cached asset if it exists."""

        for ext in ("wav", "mp3", "ogg", "flac"):
            path = self.asset_dir / f"{effect_id}.{ext}"
            if path.exists():
                return path
        return None

    def load_asset(self, effect_id: str) -> bytes:
        """Load ``effect_id`` from disk and cache it."""

        if effect_id in self.cache:
            return self.cache[effect_id]
        path = self._asset_path(effect_id)
        if path is None:
            raise FileNotFoundError(f"Audio asset '{effect_id}' not found in {self.asset_dir}")
        data = path.read_bytes()
        self.cache[effect_id] = data
        return data

    async def _generate_sound(self, effect_id: str, options: Dict[str, object]) -> bytes:
        """Delegate to the generative service to create audio."""

        return await self.generative_service(effect_id, options)

    async def _get_sound(self, effect_id: str, options: Dict[str, object]) -> bytes:
        """Retrieve audio data according to the configured mode."""

        if self.mode == "assets":
            return self.load_asset(effect_id)
        if self.mode == "generation":
            return await self._generate_sound(effect_id, options)
        if self.mode == "hybrid":
            try:
                return self.load_asset(effect_id)
            except FileNotFoundError:
                return await self._generate_sound(effect_id, options)
        raise ValueError(f"Unsupported audio mode: {self.mode}")

    # ------------------------------------------------------------------
    async def play(
        self,
        effect_id: str,
        *,
        volume: float = 1.0,
        loops: int = 0,
        channel: str = "master",
        **options: object,
    ) -> None:
        """Play a sound effect.

        Parameters
        ----------
        effect_id:
            Identifier of the effect.  For asset based playback this is the
            file name without extension.  For generation it is passed to the
            generative service.
        volume:
            Floating point value between 0 and 1.
        loops:
            Number of times the sound should repeat after the first play.
        channel:
            Arbitrary label to organise sounds.  The engine does not interpret
            channel names but stores playback history grouped by them.
        options:
            Additional keyword arguments forwarded to the generative service.
        """

        data = await self._get_sound(effect_id, options)
        # Real playback would be done here using a library like pygame or
        # simpleaudio.  To keep the project lightweight and deterministic for
        # tests we merely record the playback request.
        self.channels.setdefault(channel, []).append((effect_id, volume, loops))
        self.logger.info(
            "Playing sound %s on channel %s (volume=%s loops=%s) [%d bytes]",
            effect_id,
            channel,
            volume,
            loops,
            len(data),
        )


__all__ = ["SoundEngine"]
