from __future__ import annotations

"""Utilities for extracting identifiers and generating ``@neyra`` annotations.

The :class:`TranslationManager` parses Python source code, looks up translations
for identifiers and ensures that corresponding ``@neyra`` comments are present or
updated.  A simple dictionary based translator is bundled while an optional
``ai_service`` callback may be supplied for external translation services.
"""

from dataclasses import dataclass
import ast
import re
from typing import Callable, Dict, List, Optional


@dataclass(frozen=True)
class Identifier:
    """Identifier extracted from source code.

    Attributes
    ----------
    name:
        The identifier name as it appears in code.
    lineno:
        Line number where the identifier is defined.
    kind:
        ``"var"`` for variables/arguments or ``"visual_block"`` for function and
        class definitions.
    """

    name: str
    lineno: int
    kind: str


class TranslationManager:
    """Handle identifier translation and ``@neyra`` annotation generation."""

    def __init__(
        self,
        dictionary: Optional[Dict[str, str]] = None,
        ai_service: Optional[Callable[[str, str], str]] = None,
    ) -> None:
        self.dictionary: Dict[str, str] = dictionary or {}
        self.ai_service = ai_service

    # ------------------------------------------------------------------ Extract
    def extract_identifiers(self, code: str) -> List[Identifier]:
        """Return identifiers found in ``code``.

        Only Python syntax is currently supported.
        """

        tree = ast.parse(code)
        identifiers: List[Identifier] = []

        for node in ast.walk(tree):
            if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)):
                identifiers.append(Identifier(node.name, node.lineno, "visual_block"))
                args = getattr(node, "args", None)
                if args:
                    for arg in list(args.args) + list(args.kwonlyargs):
                        identifiers.append(Identifier(arg.arg, arg.lineno, "var"))
            elif isinstance(node, ast.Assign):
                for target in node.targets:
                    if isinstance(target, ast.Name):
                        identifiers.append(Identifier(target.id, target.lineno, "var"))
            elif isinstance(node, ast.AnnAssign):
                if isinstance(node.target, ast.Name):
                    identifiers.append(Identifier(node.target.id, node.lineno, "var"))

        # Deduplicate while preserving order
        seen: set[tuple[str, int, str]] = set()
        unique: List[Identifier] = []
        for ident in identifiers:
            key = (ident.name, ident.lineno, ident.kind)
            if key not in seen:
                seen.add(key)
                unique.append(ident)
        return unique

    # --------------------------------------------------------------- Translation
    def translate(self, identifier: str, lang: str) -> Optional[str]:
        """Return translation for ``identifier`` in ``lang``.

        The lookup first checks the provided ``dictionary``.  When a translation
        is missing and ``ai_service`` is supplied it will be invoked.  The
        ``ai_service`` callable must accept ``identifier`` and ``lang`` and return
        the translated string.
        """

        if identifier in self.dictionary:
            return self.dictionary[identifier]
        if self.ai_service is not None:
            try:
                return self.ai_service(identifier, lang)
            except Exception:
                return None
        return None

    @staticmethod
    def generate_name(translation: str) -> str:
        """Generate a Python identifier from ``translation``."""

        return re.sub(r"\W+", "_", translation.strip().lower()).strip("_")

    @staticmethod
    def reverse_translate_name(name: str) -> str:
        """Convert ``name`` back to a human readable form."""

        return name.replace("_", " ").strip().title()

    # --------------------------------------------------------------- Annotation
    def annotate_source(self, source: str, lang: str = "en") -> str:
        """Insert or update ``@neyra`` comments in ``source``.

        Only identifiers with available translations are annotated.  Existing
        comments are updated in-place while new ones are inserted directly above
        the corresponding line preserving indentation.
        """

        identifiers = sorted(self.extract_identifiers(source), key=lambda i: i.lineno)
        lines = source.splitlines()
        offset = 0

        for ident in identifiers:
            translation = self.translate(ident.name, lang)
            if not translation:
                continue

            idx = ident.lineno - 1 + offset
            comment = (
                f"@neyra:{ident.kind} id=\"{ident.name}\" display=\"{self._escape(translation)}\""
            )

            if idx > 0 and lines[idx - 1].lstrip().startswith(f"# @neyra:{ident.kind}"):
                # Update existing comment
                line = lines[idx - 1]
                indent = line[: len(line) - len(line.lstrip())]
                content = line.lstrip()[2:] if line.lstrip().startswith("# ") else line.lstrip()[1:]
                tokens = content.split()
                prefix = tokens[0]
                attrs = {}
                for token in tokens[1:]:
                    if "=" in token:
                        k, v = token.split("=", 1)
                        attrs[k] = v
                attrs["display"] = f'"{self._escape(translation)}"'
                attr_str = " ".join(f"{k}={v}" for k, v in attrs.items())
                lines[idx - 1] = f"{indent}# {prefix} {attr_str}"
            else:
                # Insert new comment
                indent = re.match(r"\s*", lines[idx] if idx < len(lines) else "").group(0)
                lines.insert(idx, f"{indent}# {comment}")
                offset += 1

        return "\n".join(lines) + ("\n" if source.endswith("\n") else "")

    @staticmethod
    def _escape(value: str) -> str:
        return value.replace('"', '\\"')
