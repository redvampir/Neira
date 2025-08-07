from __future__ import annotations

"""Helpers for adapting text to a preferred language.

The module provides lightweight utilities to automatically detect the
language of a piece of text and, when possible, translate it to a target
language.  The implementation is deliberately defensive – if an external
translator library is unavailable or translation fails for any reason, the
original text is returned unchanged.

Two high level helpers are exposed:

``adapt_request``
    Detects the language of an incoming request and translates it to the
    desired language (Russian by default).  The function returns a tuple with
    the detected language and the potentially translated text.

``adapt_response``
    Translates a response from the internal language (again Russian by
    default) back to the language of the original request.
"""

from typing import Tuple, Protocol

from .lang_quality import detect_language

try:  # pragma: no cover - external dependency may be missing
    from googletrans import Translator as _GoogleTranslator  # type: ignore
except Exception:  # pragma: no cover - handled gracefully
    _GoogleTranslator = None  # type: ignore


class _TranslatorProtocol(Protocol):
    """Simple protocol matching the subset of googletrans we rely on."""

    def translate(self, text: str, src: str, dest: str) -> object:
        ...


def _get_translator(translator: _TranslatorProtocol | None) -> _TranslatorProtocol | None:
    """Return a translator instance or ``None`` if translation isn't available."""

    if translator is not None:
        return translator
    if _GoogleTranslator is None:  # pragma: no cover - depends on external lib
        return None
    try:  # pragma: no cover - constructor may raise
        return _GoogleTranslator()
    except Exception:  # pragma: no cover - fall back when instantiation fails
        return None


def _translate(
    text: str,
    src: str,
    dest: str,
    translator: _TranslatorProtocol | None = None,
) -> str:
    """Translate ``text`` from ``src`` to ``dest`` using ``translator`` if possible."""

    if src == dest:
        return text
    translator = _get_translator(translator)
    if translator is None:
        return text
    try:  # pragma: no cover - errors from external libraries
        result = translator.translate(text, src=src, dest=dest)
    except Exception:  # pragma: no cover
        return text
    # ``googletrans`` returns an object with a ``text`` attribute.
    return getattr(result, "text", text)


def adapt_request(
    text: str,
    target_language: str = "ru",
    translator: _TranslatorProtocol | None = None,
) -> Tuple[str, str]:
    """Detect language of ``text`` and translate to ``target_language``.

    Parameters
    ----------
    text:
        Source text provided by the user.
    target_language:
        Language that the application expects internally.  Defaults to
        Russian (``"ru"``).
    translator:
        Optional translator object implementing :class:`_TranslatorProtocol`.

    Returns
    -------
    Tuple[str, str]
        Pair of detected language and the (possibly translated) text.
    """

    src_lang = detect_language(text)
    translated = _translate(text, src_lang, target_language, translator)
    return src_lang, translated


def adapt_response(
    text: str,
    original_language: str,
    source_language: str = "ru",
    translator: _TranslatorProtocol | None = None,
) -> str:
    """Translate ``text`` from ``source_language`` to ``original_language``.

    The function is meant to be used for converting a response generated in
    the application's internal language back to the language of the original
    request.  If translation is not possible, the text is returned unchanged.
    """

    return _translate(text, source_language, original_language, translator)


__all__ = ["adapt_request", "adapt_response"]
