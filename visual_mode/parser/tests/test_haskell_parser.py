from __future__ import annotations

import tempfile
from pathlib import Path

from visual_mode.parser.haskell_parser import HaskellParser


def test_extract_comments_line_and_block():
    src = """module Sample where

-- Adds two numbers
add x y = x + y

{-|
 Multi line comment
 describing function.
-}
sub x y = x - y
"""
    with tempfile.TemporaryDirectory() as tmp:
        path = Path(tmp) / "sample.hs"
        path.write_text(src, encoding="utf-8")
        parser = HaskellParser()
        module = parser.parse_file(path)
        assert module.comments[4] == "Adds two numbers"
        assert module.comments[10] == "Multi line comment\ndescribing function."

