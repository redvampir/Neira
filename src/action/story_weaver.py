"""
Ткач историй - переплетаю сюжетные линии.
"""

from __future__ import annotations

import re
from typing import List


class StoryWeaver:
    """Сплетаю отдельные сцены в цельное повествование."""

    _transitions: List[str] = ["Then", "Next", "Afterward", "Meanwhile"]

    def weave(self, scenes: List[str]) -> str:
        """Комбинировать фрагменты сцен в единый рассказ.

        Метод добавляет переходы между сценами и пытается устранить
        противоречия простым анализом состояний персонажей.

        Args:
            scenes: Список фрагментов сцен.

        Returns:
            str: Сложенное повествование.
        """

        cleaned = [s.strip() for s in scenes if s and s.strip()]
        if not cleaned:
            return ""

        narrative: List[str] = []
        last_fragment = ""
        trans_idx = 0
        states: dict[str, str] = {}
        pattern = re.compile(
            r"^(?P<subject>\w+) is (?P<state>\w+)([.!?])?", re.IGNORECASE
        )

        for fragment in cleaned:
            if last_fragment and fragment.lower() == last_fragment.lower():
                # Повторяющиеся сцены удаляются
                continue

            prefix = ""
            match = pattern.match(fragment)
            if match:
                subject = match.group("subject")
                state = match.group("state")
                prev = states.get(subject)
                if prev and prev != state:
                    prefix = "However, "
                elif prev == state:
                    # состояние не изменилось - сцена избыточна
                    continue
                states[subject] = state

            if not prefix and narrative:
                prefix = self._transitions[trans_idx % len(self._transitions)] + ", "
                trans_idx += 1
            elif prefix and narrative:
                trans_idx += 1

            # Обеспечить корректное окончание сцены
            if fragment[-1] not in ".!?":
                fragment += "."

            if prefix.endswith(", "):
                base = prefix[:-2]
                if base in self._transitions:
                    first_word = fragment.split()[0]
                    pronouns = {"he", "she", "it", "they", "we", "i", "you"}
                    if first_word.lower() in pronouns:
                        fragment = first_word.lower() + fragment[len(first_word) :]

            narrative.append(prefix + fragment)
            last_fragment = fragment

        return " ".join(narrative)
