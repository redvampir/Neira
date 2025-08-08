from __future__ import annotations

"""Generate multiple response candidates using :class:`BaseGenerator`."""

from typing import List, Optional

from src.action.base_generator import BaseGenerator
from src.llm import BaseLLM
from src.memory.style_memory import StylePattern


class CandidateGenerator(BaseGenerator):
    """Utility that produces several variations of a response.

    The class reuses the existing :class:`~src.action.base_generator.BaseGenerator`
    infrastructure to format prompts and call the LLM.  The
    :meth:`generate_candidates` method simply invokes ``generate`` multiple
    times, relying on the LLM's stochasticity to provide diverse outputs.
    When no LLM is available, the ``fallback_text`` is returned for all
    candidates.
    """

    def __init__(
        self,
        llm: Optional[BaseLLM],
        template: str,
        num_candidates: int = 3,
    ) -> None:
        super().__init__(llm, template)
        self.num_candidates = num_candidates

    # ------------------------------------------------------------------
    def generate_candidates(
        self,
        prompt: str,
        fallback_text: str,
        max_tokens: int = 512,
        style: StylePattern | None = None,
    ) -> List[str]:
        """Generate ``num_candidates`` variations for ``prompt``.

        Parameters
        ----------
        prompt:
            Input prompt used to format the template.
        fallback_text:
            Text returned when no LLM is available.
        max_tokens:
            Maximum amount of tokens for each generation.
        style:
            Optional style pattern influencing the prompt.
        """

        return [
            super().generate(prompt, fallback_text, max_tokens=max_tokens, style=style)
            for _ in range(self.num_candidates)
        ]
