from __future__ import annotations

"""Game rules utilities."""

from pathlib import Path
import json
from typing import Any, Dict, Mapping, Union

from .dice import DiceResult

try:  # pragma: no cover - optional dependency
    import yaml
except ImportError:  # pragma: no cover - only handle missing PyYAML
    yaml = None

# Directory containing ruleset files
RULES_DIR = Path(__file__).resolve().parents[2] / "config" / "rulesets"

# Currently loaded ruleset
_ruleset: Dict[str, Any] = {}


def _load_yaml(file_obj) -> Any:
    if yaml is None:
        raise RuntimeError("YAML support requires PyYAML to be installed")
    return yaml.safe_load(file_obj)


def load_ruleset(ruleset_name: str) -> Mapping[str, Any]:
    """Load and activate *ruleset_name*.

    The loader searches for a JSON or YAML file in ``config/rulesets``
    with the given ``ruleset_name``. The loaded rules are stored
    globally so that other helper functions operate on the active
    ruleset.
    """

    for ext, loader in (("json", json.load), ("yaml", _load_yaml), ("yml", _load_yaml)):
        path = RULES_DIR / f"{ruleset_name}.{ext}"
        if path.exists():
            with path.open(encoding="utf-8") as f:
                data = loader(f)
            _ruleset.clear()
            _ruleset.update(data)
            return data
    raise FileNotFoundError(f"Ruleset '{ruleset_name}' not found in {RULES_DIR}")


def check_action_validity(action: Mapping[str, Any]) -> bool:
    """Verify that ``action`` is defined and uses allowed targets."""

    if not _ruleset:
        raise RuntimeError("No ruleset has been loaded")

    name = action.get("name")
    if not name or name not in _ruleset.get("actions", {}):
        raise ValueError(f"Unknown action: {name}")

    rule = _ruleset["actions"][name]
    valid_targets = rule.get("valid_targets")
    target = action.get("target")
    if valid_targets and target not in valid_targets:
        raise ValueError(f"Invalid target '{target}' for action '{name}'")
    return True


def calculate_difficulty(action: Mapping[str, Any], context: Mapping[str, Any] | None = None) -> int:
    """Calculate the difficulty for ``action`` in ``context``."""

    if not _ruleset:
        raise RuntimeError("No ruleset has been loaded")

    name = action.get("name")
    rule = _ruleset.get("actions", {}).get(name)
    if rule is None:
        raise ValueError(f"Unknown action: {name}")

    base = int(rule.get("base_difficulty", 0))
    modifier = int(context.get("modifier", 0)) if context else 0
    return base + modifier


def resolve_action(action: Mapping[str, Any], dice_result: Union[DiceResult, int]) -> str:
    """Resolve ``action`` based on ``dice_result`` using active rules."""

    if not _ruleset:
        raise RuntimeError("No ruleset has been loaded")

    name = action.get("name")
    rule = _ruleset.get("actions", {}).get(name)
    if rule is None:
        raise ValueError(f"Unknown action: {name}")

    thresholds = rule.get("thresholds", {})
    total = dice_result.total if isinstance(dice_result, DiceResult) else int(dice_result)
    # Determine outcome based on thresholds
    sorted_thresholds = sorted(thresholds.items(), key=lambda item: item[1])
    outcome = sorted_thresholds[0][0] if sorted_thresholds else ""
    for label, threshold in sorted_thresholds:
        if total >= threshold:
            outcome = label
        else:
            break
    return outcome
