"""Simple SVG import and export helpers."""

from typing import List, Tuple
from xml.etree import ElementTree as ET
import re

ET.register_namespace("", "http://www.w3.org/2000/svg")


def export_svg(
    vector_data: List[str], file_path: str, size: Tuple[int, int] = (100, 100)
) -> None:
    """Export SVG snippets to a file."""
    svg = ET.Element(
        "svg",
        xmlns="http://www.w3.org/2000/svg",
        width=str(size[0]),
        height=str(size[1]),
    )
    for snippet in vector_data:
        svg.append(ET.fromstring(snippet))
    tree = ET.ElementTree(svg)
    tree.write(file_path)


def _strip_namespace(snippet: str) -> str:
    snippet = re.sub(r" xmlns(:ns\d+)?=\"[^\"]+\"", "", snippet)
    snippet = re.sub(r"ns\d+:", "", snippet)
    return snippet


def import_svg(file_path: str) -> List[str]:
    """Import SVG snippets from a file."""
    tree = ET.parse(file_path)
    root = tree.getroot()
    return [_strip_namespace(ET.tostring(el, encoding="unicode")) for el in list(root)]
