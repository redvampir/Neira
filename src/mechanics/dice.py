from __future__ import annotations

"""Utility helpers for rolling dice in RPG-style notation."""

from dataclasses import dataclass
import random
import re
from typing import List, Mapping


DICE_NOTATION = re.compile(r"^\s*(\d*)d(\d+)([+-]\d+)?\s*$")


@dataclass
class DiceResult:
    """Outcome of a dice roll.

    Attributes:
        rolls: Individual dice outcomes.
        modifier: Numeric modifier applied to the total roll.
    """

    rolls: List[int]
    modifier: int = 0

    @property
    def total(self) -> int:
        """Return the final value including modifiers."""

        return sum(self.rolls) + self.modifier


def roll(notation: str) -> DiceResult:
    """Roll dice according to *notation*.

    ``notation`` should follow the common RPG format ``NdM+K`` where::

        N - number of dice (defaults to 1 if omitted)
        M - number of faces on the die
        +K/-K - optional modifier applied to the sum

    Example: ``1d20+5`` rolls one twenty-sided die and adds five.
    """

    match = DICE_NOTATION.match(notation)
    if not match:
        raise ValueError(f"Invalid dice notation: {notation}")

    count_str, sides_str, mod_str = match.groups()
    count = int(count_str) if count_str else 1
    sides = int(sides_str)
    modifier = int(mod_str) if mod_str else 0

    rolls = [random.randint(1, sides) for _ in range(count)]
    return DiceResult(rolls=rolls, modifier=modifier)


def interpret_result(result: DiceResult, thresholds: Mapping[str, int]) -> str:
    """Interpret *result* against difficulty *thresholds*.

    ``thresholds`` maps outcome labels to minimum totals required to
    achieve them. The highest threshold that does not exceed the roll's
    total is returned. For example::

        interpret_result(result, {"failure": 0, "success": 10, "critical": 20})

    will yield ``"critical"`` for totals >= 20, ``"success"`` for totals
    in [10, 19] and ``"failure"`` otherwise.
    """

    total = result.total
    sorted_thresholds = sorted(thresholds.items(), key=lambda item: item[1])
    interpretation = sorted_thresholds[0][0]
    for label, threshold in sorted_thresholds:
        if total >= threshold:
            interpretation = label
        else:
            break
    return interpretation
