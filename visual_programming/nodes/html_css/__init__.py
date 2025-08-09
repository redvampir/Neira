from __future__ import annotations

"""HTML and CSS node definitions for visual programming.

This module provides lightweight data structures to model HTML elements and
CSS rules inside the visual programming environment.  They intentionally avoid
any GUI or editor specific logic and focus solely on representing the
structure that can later be consumed by other parts of the system.
"""

from dataclasses import dataclass, field
from pathlib import Path
from typing import Dict, Iterable, List, Union

from ...code_generator import GENERATED_DIR


@dataclass
class HTMLElement:
    """Representation of an HTML element in a graph.

    Parameters
    ----------
    tag: str
        Name of the HTML tag, e.g. ``"div"`` or ``"span"``.
    attributes: Dict[str, str]
        Optional mapping of HTML attributes.
    children: List[Union[str, "HTMLElement"]]
        Nested elements or raw text contained inside the element.
    """

    tag: str
    attributes: Dict[str, str] = field(default_factory=dict)
    children: List[Union[str, "HTMLElement"]] = field(default_factory=list)

    def render(self, indent: int = 0) -> str:
        """Return the HTML representation of this element and its children."""
        pad = "  " * indent
        attrs = "".join(f' {k}="{v}"' for k, v in self.attributes.items())
        if not self.children:
            return f"{pad}<{self.tag}{attrs}/>"  # self-closing

        inner_parts: List[str] = []
        for child in self.children:
            if isinstance(child, HTMLElement):
                inner_parts.append(child.render(indent + 1))
            else:
                inner_parts.append("  " * (indent + 1) + child)
        inner = "\n".join(inner_parts)
        return f"{pad}<{self.tag}{attrs}>\n{inner}\n{pad}</{self.tag}>"


@dataclass
class CSSRule:
    """Representation of a CSS rule."""

    selector: str
    properties: Dict[str, str] = field(default_factory=dict)

    def render(self) -> str:
        """Return the CSS rule as a string."""
        props = "; ".join(f"{k}: {v}" for k, v in self.properties.items())
        if props:
            props += ";"
        return f"{self.selector} {{ {props} }}"


def export_html_css(root: HTMLElement, css_rules: Iterable[CSSRule]) -> tuple[Path, Path]:
    """Export the HTML/CSS graph to files in the generated directory.

    Parameters
    ----------
    root: HTMLElement
        The root element of the HTML document.
    css_rules: Iterable[CSSRule]
        Collection of CSS rules to serialize.

    Returns
    -------
    tuple[Path, Path]
        Paths to the written HTML and CSS files respectively.
    """

    html_path = GENERATED_DIR / "html_template.html"
    css_path = GENERATED_DIR / "style.css"

    html_path.write_text(root.render() + "\n", encoding="utf-8")
    css_content = "\n".join(rule.render() for rule in css_rules)
    css_path.write_text(css_content + ("\n" if css_content else ""), encoding="utf-8")

    return html_path, css_path


__all__ = ["HTMLElement", "CSSRule", "export_html_css"]
