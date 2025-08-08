import sys
import types

sys.modules.setdefault(
    "sentence_transformers", types.SimpleNamespace(SentenceTransformer=lambda *a, **k: None)
)

from src.analysis.post_processor import run_post_processors


def test_candidate_selection_pipeline():
    class StubCandidateGenerator:
        def generate_candidates(self, prompt: str, fallback: str):
            return [prompt, prompt.upper()]

    class StubCandidateSelector:
        def select_best(self, candidates):
            return candidates[1]

    result, corrections = run_post_processors(
        "hi", [], StubCandidateGenerator(), StubCandidateSelector()
    )
    assert result == "HI"
    assert corrections == []
