"""Utilities and base classes for visual mode language parsing."""

from .base import LanguageParser  # re-export for convenience
from . import utils

# Optional parser implementations.  They are imported lazily so that missing
# third-party dependencies do not prevent the package from being imported.
try:  # pragma: no cover - optional dependency
    from .python_parser import PythonParser  # type: ignore
except Exception:  # pragma: no cover - dependency missing
    PythonParser = None  # type: ignore

try:  # pragma: no cover - optional dependency
    from .java_parser import JavaParser  # type: ignore
except Exception:  # pragma: no cover - dependency missing
    JavaParser = None  # type: ignore

try:  # pragma: no cover - optional dependency
    from .c_parser import CParser  # type: ignore
except Exception:  # pragma: no cover - dependency missing
    CParser = None  # type: ignore

try:  # pragma: no cover - optional dependency
    from .cpp_parser import CppParser  # type: ignore
except Exception:  # pragma: no cover - dependency missing
    CppParser = None  # type: ignore

try:  # pragma: no cover - optional dependency
    from .csharp_parser import CSharpParser  # type: ignore
except Exception:  # pragma: no cover - dependency missing
    CSharpParser = None  # type: ignore

try:  # pragma: no cover - optional dependency
    from .dart_parser import DartParser  # type: ignore
except Exception:  # pragma: no cover - dependency missing
    DartParser = None  # type: ignore

try:  # pragma: no cover - optional dependency
    from .javascript_parser import JavaScriptParser  # type: ignore
except Exception:  # pragma: no cover - dependency missing
    JavaScriptParser = None  # type: ignore

try:  # pragma: no cover - optional dependency
    from .php_parser import PHPParser  # type: ignore
except Exception:  # pragma: no cover - dependency missing
    PHPParser = None  # type: ignore

try:  # pragma: no cover - optional dependency
    from .typescript_parser import TypeScriptParser  # type: ignore
except Exception:  # pragma: no cover - dependency missing
    TypeScriptParser = None  # type: ignore

try:  # pragma: no cover - optional dependency
    from .go_parser import GoParser  # type: ignore
except Exception:  # pragma: no cover - dependency missing
    GoParser = None  # type: ignore

try:  # pragma: no cover - optional dependency
    from .rust_parser import RustParser  # type: ignore
except Exception:  # pragma: no cover - dependency missing
    RustParser = None  # type: ignore

try:  # pragma: no cover - optional dependency
    from .ruby_parser import RubyParser  # type: ignore
except Exception:  # pragma: no cover - dependency missing
    RubyParser = None  # type: ignore

try:  # pragma: no cover - optional dependency
    from .swift_parser import SwiftParser  # type: ignore
except Exception:  # pragma: no cover - dependency missing
    SwiftParser = None  # type: ignore

__all__ = ["LanguageParser", "utils"]
if PythonParser is not None:
    __all__.append("PythonParser")
if JavaParser is not None:
    __all__.append("JavaParser")
if CParser is not None:
    __all__.append("CParser")
if CppParser is not None:
    __all__.append("CppParser")
if CSharpParser is not None:
    __all__.append("CSharpParser")
if DartParser is not None:
    __all__.append("DartParser")
if JavaScriptParser is not None:
    __all__.append("JavaScriptParser")
if PHPParser is not None:
    __all__.append("PHPParser")
if TypeScriptParser is not None:
    __all__.append("TypeScriptParser")
if GoParser is not None:
    __all__.append("GoParser")
if RustParser is not None:
    __all__.append("RustParser")
if RubyParser is not None:
    __all__.append("RubyParser")
if SwiftParser is not None:
    __all__.append("SwiftParser")