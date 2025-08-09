from __future__ import annotations

"""Profile management for translation dictionaries.

Profiles are stored as JSON files inside ``userdata/translation_profiles``.
Each file contains a ``priority`` integer and a ``dictionary`` mapping of
identifier → translated display name.  The active profile is determined by the
``active_profile.json`` file in the same directory or, when absent, by selecting
the profile with the highest priority.
"""

from dataclasses import dataclass
from pathlib import Path
from typing import Dict, Optional
import json

# --------------------------------------------------------------------------- Paths
ROOT_DIR = Path(__file__).resolve().parents[2]
PROFILE_DIR = ROOT_DIR / "userdata" / "translation_profiles"
ACTIVE_FILE = PROFILE_DIR / "active_profile.json"


# --------------------------------------------------------------------------- Data model
@dataclass
class Profile:
    """Representation of a translation profile."""

    name: str
    priority: int
    dictionary: Dict[str, str]


# --------------------------------------------------------------------------- Helpers

def load_profiles() -> Dict[str, Profile]:
    """Load all profiles from :data:`PROFILE_DIR`.

    Invalid or unreadable files are skipped.  The result maps profile names to
    :class:`Profile` instances.
    """

    profiles: Dict[str, Profile] = {}
    if not PROFILE_DIR.exists():
        return profiles
    for path in PROFILE_DIR.glob("*.json"):
        if path.name == ACTIVE_FILE.name:
            continue
        try:
            data = json.loads(path.read_text(encoding="utf-8"))
        except Exception:
            continue
        priority = int(data.get("priority", 0))
        dictionary = {
            str(k): str(v) for k, v in (data.get("dictionary") or {}).items()
        }
        profiles[path.stem] = Profile(path.stem, priority, dictionary)
    return profiles


def _read_active_name() -> Optional[str]:
    if ACTIVE_FILE.exists():
        try:
            data = json.loads(ACTIVE_FILE.read_text(encoding="utf-8"))
            name = data.get("name")
            if isinstance(name, str):
                return name
        except Exception:
            return None
    return None


def get_active_profile_name(profiles: Dict[str, Profile] | None = None) -> Optional[str]:
    """Return the currently active profile name.

    The explicit selection stored in ``active_profile.json`` takes precedence.
    When unset, the profile with the highest ``priority`` value is used.  The
    function returns ``None`` when no profiles are available.
    """

    if profiles is None:
        profiles = load_profiles()
    selected = _read_active_name()
    if selected and selected in profiles:
        return selected
    if profiles:
        return max(profiles.values(), key=lambda p: p.priority).name
    return None


def set_active_profile(name: str) -> None:
    """Persist ``name`` as the active profile."""

    PROFILE_DIR.mkdir(parents=True, exist_ok=True)
    ACTIVE_FILE.write_text(json.dumps({"name": name}), encoding="utf-8")



def get_active_profile() -> Profile:
    """Return the active :class:`Profile` instance.

    When no profiles exist, an empty profile is returned allowing callers to
    operate with an empty dictionary.
    """

    profiles = load_profiles()
    name = get_active_profile_name(profiles)
    if name and name in profiles:
        return profiles[name]
    return Profile(name or "", 0, {})


__all__ = [
    "Profile",
    "load_profiles",
    "get_active_profile",
    "get_active_profile_name",
    "set_active_profile",
]
