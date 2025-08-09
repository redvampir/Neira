from __future__ import annotations

"""Haskell source parser for visual programming mode.

This parser attempts to use the ``haskell-src-exts`` library to extract
comment metadata from Haskell source files.  Line ``--`` comments and block
``{- ... -}`` comments are associated with the line of code that follows them
(similar to other language parsers in this package).  When the external
``runghc`` command or required Haskell packages are not available the parser
falls back to a lightweight Python implementation that performs a best effort
comment extraction.
"""

from dataclasses import dataclass
import json
import shutil
import subprocess
from pathlib import Path
from typing import Any, Dict, Iterable, List

from .base import LanguageParser

# ---------------------------------------------------------------------------
# Haskell helper program
# ---------------------------------------------------------------------------

HASKELL_HELPER = r"""
{-# LANGUAGE OverloadedStrings #-}
import System.Environment (getArgs)
import Language.Haskell.Exts
    ( Comment(..)
    , ParseResult(..)
    , defaultParseMode
    , parseModuleWithComments
    )
import Language.Haskell.Exts.SrcLoc (srcSpanStartLine)
import Data.Aeson (encode, object, (.=))
import qualified Data.ByteString.Lazy.Char8 as BL

main :: IO ()
main = do
    [path] <- getArgs
    src <- readFile path
    case parseModuleWithComments defaultParseMode src of
      ParseOk (_, comments) ->
        BL.putStrLn $ encode [ object [ "line" .= srcSpanStartLine l
                                      , "text" .= t ]
                             | Comment _ l t <- comments ]
      ParseFailed _ err -> error err
"""

# ---------------------------------------------------------------------------
# Utilities
# ---------------------------------------------------------------------------

def _clean_comment_text(text: str) -> str:
    """Normalize comment text by stripping decorations."""

    text = text.strip()
    if text.startswith("--"):
        text = text[2:]
    if text.startswith("{-") and text.endswith("-}"):
        text = text[2:-2]
    lines = [ln.strip().lstrip("*").lstrip("|") for ln in text.splitlines()]
    return "\n".join([ln.strip() for ln in lines]).strip()

def _extract_comments_hse(path: Path) -> Dict[int, str]:
    """Extract comments using ``haskell-src-exts`` via ``runghc``."""

    runghc = shutil.which("runghc")
    if not runghc:
        raise RuntimeError("runghc not found")
    try:
        proc = subprocess.run(
            [runghc, "-ignore-dot-ghci", "-", str(path)],
            input=HASKELL_HELPER.encode("utf-8"),
            capture_output=True,
            text=True,
            check=True,
        )
    except Exception as exc:  # pragma: no cover - optional dependency
        raise RuntimeError("failed to run haskell parser") from exc
    data = json.loads(proc.stdout or "[]")
    comments: Dict[int, str] = {}
    for item in data:
        line = int(item["line"])
        comments[line] = _clean_comment_text(item["text"])
    return comments

def _extract_comments_fallback(source: str) -> Dict[int, str]:
    """Fallback comment extraction implemented in pure Python."""

    comments: Dict[int, str] = {}
    lines = source.splitlines()
    pending: List[str] = []
    i = 0
    while i < len(lines):
        line = lines[i]
        stripped = line.strip()
        # full line -- comments
        if stripped.startswith("--"):
            pending.append(stripped[2:].strip())
            i += 1
            continue
        # block comments {- -}
        if stripped.startswith("{-"):
            block = line[line.find("{-") + 2 :]
            j = i
            end = line.find("-}")
            while end == -1 and j + 1 < len(lines):
                j += 1
                block += "\n" + lines[j]
                end = lines[j].find("-}")
            if end != -1:
                block_content = block[: block.rfind("-}")]
            else:
                block_content = block
            pending.append(_clean_comment_text(block_content))
            i = j + 1
            continue
        # inline comments
        inline: str | None = None
        if "--" in line:
            idx = line.find("--")
            if line[:idx].strip():
                inline = line[idx + 2 :].strip()
        if inline is None and "{-" in line and "-}" in line:
            start = line.find("{-")
            end = line.find("-}", start + 2)
            if line[:start].strip():
                inline = _clean_comment_text(line[start + 2 : end])
        if inline is not None:
            comments[i + 1] = inline
            pending = []
            i += 1
            continue
        if stripped:
            if pending:
                comments[i + 1] = "\n".join(pending).strip()
                pending = []
        i += 1
    return comments

# ---------------------------------------------------------------------------
# Public API
# ---------------------------------------------------------------------------

@dataclass
class ParsedHaskell:
    """Container holding parsed information about a Haskell module."""

    tree: Any | None
    source: str
    comments: Dict[int, str]


class HaskellParser(LanguageParser):
    """Concrete :class:`LanguageParser` implementation for Haskell."""

    def parse_file(self, path: str | Path) -> ParsedHaskell:
        path = Path(path)
        source = path.read_text(encoding="utf-8")
        try:
            comments = _extract_comments_hse(path)
        except Exception:  # pragma: no cover - optional dependency
            comments = _extract_comments_fallback(source)
        return ParsedHaskell(tree=None, source=source, comments=comments)

    def extract_nodes(self, module: ParsedHaskell) -> Iterable[Dict[str, Any]]:
        return []

    def extract_connections(self, module: ParsedHaskell) -> Iterable[Any]:
        return []

