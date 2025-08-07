import pytest

from src.core.session_scoped_memory import SessionScopedMemory
from src.models import Character


def test_returns_separate_instances_per_user(tmp_path):
    sm = SessionScopedMemory(base_path=tmp_path)
    c1, w1, s1 = sm.get("u1")
    c2, w2, s2 = sm.get("u2")
    assert c1 is not c2
    assert w1 is not w2
    assert s1 is not s2
    assert "u1" in str(c1.storage_path)
    assert "u2" in str(c2.storage_path)


def test_caches_instances(tmp_path):
    sm = SessionScopedMemory(base_path=tmp_path)
    c1, w1, s1 = sm.get("u1")
    c1b, w1b, s1b = sm.get("u1")
    assert c1 is c1b
    assert w1 is w1b
    assert s1 is s1b


def test_memories_are_isolated(tmp_path):
    sm = SessionScopedMemory(base_path=tmp_path)
    c1, _, _ = sm.get("u1")
    c2, _, _ = sm.get("u2")
    alice = Character(name="Alice")
    c1.add(alice)
    assert c2.get("Alice") is None
