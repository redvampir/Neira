from __future__ import annotations

"""Kotlin source parser based on Kotlin compiler PSI.

This module provides :class:`KotlinParser` which delegates to a small Kotlin
helper program that uses the official Kotlin compiler's PSI to inspect a source
file.  The helper extracts top level functions and properties together with
KDoc and inline ``//`` comments.  The collected information is fed back into
Python as plain dictionaries that can be consumed by the visual programming
mode.

The helper program is lazily built on first use.  It requires the Kotlin
compiler which is downloaded automatically if not present.
"""

from dataclasses import dataclass
from pathlib import Path
from typing import Any, Dict, Iterable, List
import subprocess
import tempfile
import urllib.request
import zipfile
import os
from .base import LanguageParser

# Location of the Kotlin compiler and helper jar relative to this file
_COMPILER_VERSION = "1.9.0"
_BASE_DIR = Path(__file__).resolve().parent
# The official Kotlin compiler distribution extracts into a directory named
# ``kotlinc`` regardless of the version number.
_COMPILER_DIR = _BASE_DIR / "kotlinc"
_HELPER_JAR = _BASE_DIR / "_kotlin_parser_helper.jar"

# Kotlin source code for the PSI based helper.  The program prints each
# discovered declaration on its own line in a tab separated format:
#   name <TAB> type <TAB> doc <TAB> startLine <TAB> startCol <TAB> endLine <TAB> endCol
_HELPER_SRC = r"""
import org.jetbrains.kotlin.cli.common.CLIConfigurationKeys
import org.jetbrains.kotlin.cli.common.messages.MessageCollector
import org.jetbrains.kotlin.cli.jvm.compiler.EnvironmentConfigFiles
import org.jetbrains.kotlin.cli.jvm.compiler.KotlinCoreEnvironment
import org.jetbrains.kotlin.config.CompilerConfiguration
import org.jetbrains.kotlin.kdoc.psi.api.KDoc
import org.jetbrains.kotlin.psi.*
import org.jetbrains.kotlin.psi.psiUtil.collectDescendantsOfType
import com.intellij.openapi.util.Disposer
import com.intellij.psi.PsiComment

fun main(args: Array<String>) {
    val path = args[0]
    val code = java.io.File(path).readText()
    val disposable = Disposer.newDisposable()
    val configuration = CompilerConfiguration()
    configuration.put(CLIConfigurationKeys.MESSAGE_COLLECTOR_KEY, MessageCollector.NONE)
    val environment = KotlinCoreEnvironment.createForProduction(
        disposable, configuration, EnvironmentConfigFiles.JVM_CONFIG_FILES
    )
    val psiFactory = KtPsiFactory(environment.project, false)
    val ktFile = psiFactory.createFile(code)
    val document = ktFile.viewProvider.document!!

    val inlineComments = mutableMapOf<Int, String>()
    for (comment in ktFile.collectDescendantsOfType<PsiComment>()) {
        if (comment is KDoc) continue
        val text = comment.text
        if (text.startsWith("//")) {
            val line = document.getLineNumber(comment.textRange.startOffset) + 1
            inlineComments[line] = text.removePrefix("//").trim()
        }
    }

    fun rangeOf(decl: KtDeclaration): String {
        val startOffset = decl.textRange.startOffset
        val endOffset = decl.textRange.endOffset
        val startLine = document.getLineNumber(startOffset) + 1
        val endLine = document.getLineNumber(endOffset) + 1
        val startCol = startOffset - document.getLineStartOffset(startLine - 1) + 1
        val endCol = endOffset - document.getLineStartOffset(endLine - 1) + 1
        return "$startLine\t$startCol\t$endLine\t$endCol"
    }

    val out = StringBuilder()
    fun appendNode(name: String, type: String, doc: String, decl: KtDeclaration) {
        val cleaned = doc.replace("\t", " ").replace("\n", " ")
        val range = rangeOf(decl)
        out.append("$name\t$type\t$cleaned\t$range\n")
    }

    for (decl in ktFile.declarations) {
        when (decl) {
            is KtNamedFunction -> {
                val name = decl.name ?: continue
                val startLine = document.getLineNumber(decl.textRange.startOffset) + 1
                var doc = inlineComments[startLine] ?: ""
                if (doc.isEmpty()) {
                    doc = decl.docComment?.getDefaultSection()?.getContent()?.trim() ?: ""
                }
                appendNode(name, "block", doc, decl)
            }
            is KtProperty -> {
                val name = decl.name ?: continue
                val startLine = document.getLineNumber(decl.textRange.startOffset) + 1
                var doc = inlineComments[startLine] ?: ""
                if (doc.isEmpty()) {
                    doc = decl.docComment?.getDefaultSection()?.getContent()?.trim() ?: ""
                }
                appendNode(name, "variable", doc, decl)
            }
        }
    }

    print(out.toString())
    Disposer.dispose(disposable)
}
"""


@dataclass
class ParsedKotlin:
    """Container for parsed Kotlin declarations."""

    nodes: List[Dict[str, Any]]


def _download_compiler() -> None:
    """Download the Kotlin compiler distribution if not present."""
    if _COMPILER_DIR.exists():
        return
    url = (
        f"https://github.com/JetBrains/kotlin/releases/download/v{_COMPILER_VERSION}/"
        f"kotlin-compiler-{_COMPILER_VERSION}.zip"
    )
    with urllib.request.urlopen(url) as resp:
        data = resp.read()
    with tempfile.NamedTemporaryFile(delete=False) as tmp:
        tmp.write(data)
        tmp_path = Path(tmp.name)
    with zipfile.ZipFile(tmp_path) as zf:
        zf.extractall(_BASE_DIR)
    tmp_path.unlink()


def _build_helper() -> None:
    """Compile the Kotlin helper jar using ``kotlinc``."""
    _download_compiler()
    src_path = _BASE_DIR / "_KotlinPsiParser.kt"
    src_path.write_text(_HELPER_SRC)
    kotlinc = _COMPILER_DIR / "bin" / "kotlinc"
    if os.name == "nt":
        kotlinc = kotlinc.with_suffix(".bat")
    if not os.access(kotlinc, os.X_OK):  # Ensure the compiler script is executable
        os.chmod(kotlinc, 0o755)
    compiler_cp = _COMPILER_DIR / "lib" / "kotlin-compiler.jar"
    subprocess.run(
        [
            str(kotlinc),
            "-classpath",
            str(compiler_cp),
            "-include-runtime",
            "-d",
            str(_HELPER_JAR),
            str(src_path),
        ],
        check=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    src_path.unlink(missing_ok=True)


def _ensure_helper() -> None:
    if not _HELPER_JAR.exists():
        _build_helper()


class KotlinParser(LanguageParser):
    """Concrete :class:`LanguageParser` implementation for Kotlin."""

    def parse_file(self, path: str | Path) -> ParsedKotlin:
        _ensure_helper()
        compiler_cp = _COMPILER_DIR / "lib" / "kotlin-compiler.jar"
        classpath = os.pathsep.join([str(_HELPER_JAR), str(compiler_cp)])
        result = subprocess.run(
            [
                "java",
                "-classpath",
                classpath,
                "_KotlinPsiParserKt",
                str(path),
            ],
            check=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
        )
        nodes: List[Dict[str, Any]] = []
        for line in result.stdout.strip().splitlines():
            parts = line.split("\t")
            if len(parts) != 7:
                continue
            name, typ, doc, sl, sc, el, ec = parts
            nodes.append(
                {
                    "id": name,
                    "type": typ,
                    "display": doc,
                    "range": {
                        "start": {"line": int(sl), "column": int(sc)},
                        "end": {"line": int(el), "column": int(ec)},
                    },
                }
            )
        return ParsedKotlin(nodes=nodes)

    def extract_nodes(self, module: ParsedKotlin) -> Iterable[Dict[str, Any]]:
        return module.nodes

    def extract_connections(self, module: ParsedKotlin) -> Iterable[Any]:
        return []
