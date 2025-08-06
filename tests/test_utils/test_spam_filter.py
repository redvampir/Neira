"""Tests for the spam filter utility."""

from src.utils.spam_filter import is_spam


def test_detects_common_spam_phrases() -> None:
    assert is_spam("Buy now and get one free!")
    assert is_spam("Click here to win a prize")


def test_allows_regular_text() -> None:
    assert not is_spam("Python is a programming language.")
    assert not is_spam("The quick brown fox jumps over the lazy dog")
