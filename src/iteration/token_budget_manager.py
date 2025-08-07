from __future__ import annotations

"""Utility to distribute available tokens between iteration stages."""

from dataclasses import dataclass


@dataclass
class TokenBudgetManager:
    """Manage allocation of tokens for the iteration pipeline.

    Parameters
    ----------
    total_tokens:
        Maximum number of tokens available for the whole operation.
    draft_ratio, search_ratio, refine_ratio:
        Fractions of ``total_tokens`` reserved for the draft generation,
        search prompts and refinement steps respectively.  The ratios must sum
        to ``<= 1``.
    per_result_tokens:
        Estimated token cost for including one search result snippet in a
        prompt.  This is used to convert the search budget into a limit on the
        number of search results to request.
    """

    total_tokens: int
    draft_ratio: float = 0.5
    search_ratio: float = 0.3
    refine_ratio: float = 0.2
    per_result_tokens: int = 50

    def __post_init__(self) -> None:
        if self.total_tokens <= 0:
            raise ValueError("total_tokens must be positive")
        if self.draft_ratio + self.search_ratio + self.refine_ratio > 1.0:
            raise ValueError("token ratios must sum to 1 or less")

    # Draft -----------------------------------------------------------------
    @property
    def draft_tokens(self) -> int:
        return max(int(self.total_tokens * self.draft_ratio), 1)

    # Search ----------------------------------------------------------------
    def search_tokens(self) -> int:
        return max(int(self.total_tokens * self.search_ratio), 1)

    def tokens_per_search_query(self, num_queries: int = 1) -> int:
        return max(self.search_tokens() // max(num_queries, 1), 1)

    def search_limit(self, num_queries: int = 1) -> int:
        tokens = self.tokens_per_search_query(num_queries)
        return max(tokens // self.per_result_tokens, 1)

    # Refine ---------------------------------------------------------------
    @property
    def refine_tokens(self) -> int:
        return max(int(self.total_tokens * self.refine_ratio), 1)


__all__ = ["TokenBudgetManager"]
