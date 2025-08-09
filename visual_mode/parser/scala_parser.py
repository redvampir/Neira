from __future__ import annotations

"""Scala source parser using Scala's parser combinators.

This module implements :class:`ScalaParser` which delegates parsing of Scala
source files to a small Scala helper program built with the
``scala-parser-combinators`` library.  The helper extracts top level classes
and methods together with preceding Scaladoc comments, inline ``//`` comments
and ``@`` annotations.  The collected information is returned to Python as
plain dictionaries suitable for consumption by the visual programming mode.
"""

from dataclasses import dataclass
from pathlib import Path
from typing import Any, Dict, Iterable, List
import os
import subprocess
import tempfile
import urllib.request
import tarfile

from .base import LanguageParser

# ---------------------------------------------------------------------------
# Constants and helper program source
# ---------------------------------------------------------------------------

_SCALA_VERSION = "2.13.11"
_COMBINATORS_VERSION = "2.1.1"
_BASE_DIR = Path(__file__).resolve().parent
_SCALA_DIR = _BASE_DIR / "scala"
_PARSER_JAR = _BASE_DIR / "scala-parser-combinators.jar"
_HELPER_DIR = _BASE_DIR / "_scala_parser_helper"

# Scala helper program.  It prints each discovered declaration as a tab
# separated line:
#   name <TAB> type <TAB> display <TAB> startLine <TAB> startCol <TAB> endLine <TAB> endCol
_HELPER_SRC = r"""
import scala.util.parsing.combinator._
import scala.io.Source

object _ScalaParserHelper extends RegexParsers {
  override val skipWhitespace = true
  def ident: Parser[String] = "[A-Za-z_][A-Za-z0-9_]*".r
  def classDecl: Parser[(String,String)] = "class" ~> ident ^^ { n => ("block", n) }
  def defDecl: Parser[(String,String)] = "def" ~> ident ^^ { n => ("block", n) }
  def decl: Parser[(String,String)] = classDecl | defDecl

  def main(args: Array[String]): Unit = {
    val lines = Source.fromFile(args(0)).getLines().toVector
    for ((line, idx) <- lines.zipWithIndex) {
      parseAll(decl, line.trim) match {
        case Success((kind, name), _) =>
          var doc = ""
          val inlineIdx = line.indexOf("//")
          if (inlineIdx >= 0) doc = line.substring(inlineIdx + 2).trim
          var j = idx - 1
          var annotations = List[String]()
          while (j >= 0 && lines(j).trim.startsWith("@")) {
            val annLine = lines(j).trim.drop(1)
            val ann = annLine.takeWhile(c => c.isLetterOrDigit || c == '_')
            annotations = ann :: annotations
            j -= 1
          }
          if (doc.isEmpty && j >= 0 && lines(j).trim.startsWith("//")) {
            doc = lines(j).trim.stripPrefix("//").trim
          }
          if (doc.isEmpty && j >= 0 && lines(j).trim.startsWith("/**") && lines(j).trim.endsWith("*/")) {
            val raw = lines(j).trim.stripPrefix("/**").stripSuffix("*/")
            doc = raw.trim
          }
          val display = (annotations.mkString(" ") + " " + doc).trim
          val l = idx + 1
          val startCol = line.indexOf(name) + 1
          val endCol = line.length + 1
          println(s"$name\t$kind\t$display\t$l\t$startCol\t$l\t$endCol")
        case _ =>
      }
    }
  }
}
"""


@dataclass
class ParsedScala:
  """Container for parsed Scala declarations."""

  nodes: List[Dict[str, Any]]


# ---------------------------------------------------------------------------
# Build helper
# ---------------------------------------------------------------------------

def _download_scala() -> None:
  """Download the Scala distribution if not already present."""
  if _SCALA_DIR.exists():
    return
  url = f"https://downloads.lightbend.com/scala/{_SCALA_VERSION}/scala-{_SCALA_VERSION}.tgz"
  with urllib.request.urlopen(url) as resp:
    data = resp.read()
  with tempfile.NamedTemporaryFile(delete=False) as tmp:
    tmp.write(data)
    tmp_path = Path(tmp.name)
  with tarfile.open(tmp_path) as tf:
    tf.extractall(_BASE_DIR)
  ( _BASE_DIR / f"scala-{_SCALA_VERSION}" ).rename(_SCALA_DIR)
  tmp_path.unlink()


def _download_parser_combinators() -> None:
  if _PARSER_JAR.exists():
    return
  url = (
    "https://repo1.maven.org/maven2/org/scala-lang/modules/"
    f"scala-parser-combinators_2.13/{_COMBINATORS_VERSION}/"
    f"scala-parser-combinators_2.13-{_COMBINATORS_VERSION}.jar"
  )
  with urllib.request.urlopen(url) as resp:
    data = resp.read()
  _PARSER_JAR.write_bytes(data)


def _build_helper() -> None:
  _download_scala()
  _download_parser_combinators()
  if _HELPER_DIR.exists():
    return
  _HELPER_DIR.mkdir(parents=True, exist_ok=True)
  src_path = _BASE_DIR / "_ScalaParserHelper.scala"
  src_path.write_text(_HELPER_SRC)
  scalac = _SCALA_DIR / "bin" / "scalac"
  if os.name == "nt":
    scalac = scalac.with_suffix(".bat")
  if not os.access(scalac, os.X_OK):
    os.chmod(scalac, 0o755)
  subprocess.run(
    [
      str(scalac),
      "-classpath",
      str(_PARSER_JAR),
      "-d",
      str(_HELPER_DIR),
      str(src_path),
    ],
    check=True,
    stdout=subprocess.PIPE,
    stderr=subprocess.PIPE,
  )
  src_path.unlink(missing_ok=True)


def _ensure_helper() -> None:
  if not _HELPER_DIR.exists():
    _build_helper()


# ---------------------------------------------------------------------------
# Parser implementation
# ---------------------------------------------------------------------------


class ScalaParser(LanguageParser):
  """Concrete :class:`LanguageParser` implementation for Scala."""

  def parse_file(self, path: str | Path) -> ParsedScala:
    _ensure_helper()
    scala_exec = _SCALA_DIR / "bin" / "scala"
    if os.name == "nt":
      scala_exec = scala_exec.with_suffix(".bat")
    if not os.access(scala_exec, os.X_OK):
      os.chmod(scala_exec, 0o755)
    classpath = os.pathsep.join([str(_HELPER_DIR), str(_PARSER_JAR)])
    result = subprocess.run(
      [str(scala_exec), "-classpath", classpath, "_ScalaParserHelper", str(path)],
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
      name, typ, display, sl, sc, el, ec = parts
      node = {
        "id": name,
        "type": typ,
        "display": display,
        "range": {
          "start": {"line": int(sl), "column": int(sc)},
          "end": {"line": int(el), "column": int(ec)},
        },
      }
      nodes.append(node)
    return ParsedScala(nodes=nodes)

  def extract_nodes(self, module: ParsedScala) -> Iterable[Dict[str, Any]]:
    return module.nodes

  def extract_connections(self, module: ParsedScala) -> Iterable[Any]:
    return []
