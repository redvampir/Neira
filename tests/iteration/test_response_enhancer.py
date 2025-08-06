from src.iteration import ResponseEnhancer, IntegrationType


def test_critical_correction_replaces_text():
    enhancer = ResponseEnhancer()
    draft = "Earth is flat"
    results = [{"content": "Earth is round"}]
    final = enhancer.enhance(draft, results, IntegrationType.CRITICAL_CORRECTION)
    assert final == "Earth is round"


def test_important_addition_and_self_corrector():
    enhancer = ResponseEnhancer()
    draft = "teh ocean is vast"
    results = [{"content": "It covers 70% of Earth"}]
    final = enhancer.enhance(draft, results, IntegrationType.IMPORTANT_ADDITION)
    assert final.startswith("the ocean is vast")
    assert "It covers 70% of Earth" in final
    assert "teh" not in final


def test_context_enrichment_without_correction():
    enhancer = ResponseEnhancer()
    draft = "I like cats"
    results = [{"content": "Cats are mammals"}]
    final = enhancer.enhance(
        draft, results, IntegrationType.CONTEXT_ENRICHMENT, self_correct=False
    )
    assert final.startswith("Cats are mammals")
    assert final.endswith("I like cats")
