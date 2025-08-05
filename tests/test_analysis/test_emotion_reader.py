"""Tests for the emotion reader."""

import sys
from pathlib import Path

# Ensure project root is on the path for src layout
sys.path.append(str(Path(__file__).resolve().parents[2]))

from src.analysis.emotion_reader import EmotionReader


def test_detects_joy() -> None:
    reader = EmotionReader()
    scores = reader.analyze_text("I am so happy and joyful today!")
    assert scores["joy"] == 1.0
    assert scores["sadness"] == 0.0


def test_detects_sadness() -> None:
    reader = EmotionReader()
    scores = reader.analyze_text("She felt sad and miserable, wanting to cry.")
    assert scores["sadness"] == 1.0
    assert scores["joy"] < scores["sadness"]


def test_detects_anger() -> None:
    reader = EmotionReader()
    scores = reader.analyze_text("He was angry and furious, filled with rage.")
    assert scores["anger"] == 1.0
    assert scores["joy"] < scores["anger"]


def test_detects_fear() -> None:
    reader = EmotionReader()
    scores = reader.analyze_text("They felt scared and terrified by the dark.")
    assert scores["fear"] == 1.0
    assert scores["joy"] < scores["fear"]


def test_handles_negation() -> None:
    reader = EmotionReader()
    scores = reader.analyze_text("I am not happy about this.")
    assert scores["joy"] == 0.0
