from src.iteration import ResponseEnhancer, IntegrationType


def test_critical_correction_replaces_text():
    enhancer = ResponseEnhancer()
    draft = "Earth is flat"
    results = [{"content": "Earth is round"}]
    final = enhancer.enhance(draft, results, IntegrationType.CRITICAL_CORRECTION)
    assert final["text"] == "Earth is round"
    assert final["rules_refs"] == []


def test_important_addition_and_self_corrector():
    enhancer = ResponseEnhancer()
    draft = "teh ocean is vast"
    results = [{"content": "It covers 70% of Earth"}]
    final = enhancer.enhance(draft, results, IntegrationType.IMPORTANT_ADDITION)
    text = final["text"]
    assert text.startswith("the ocean is vast")
    assert "It covers 70% of Earth" in text
    assert "teh" not in text
    assert final["rules_refs"] == []


def test_context_enrichment_without_correction():
    enhancer = ResponseEnhancer()
    draft = "I like cats"
    results = [{"content": "Cats are mammals"}]
    final = enhancer.enhance(
        draft, results, IntegrationType.CONTEXT_ENRICHMENT, self_correct=False
    )
    text = final["text"]
    assert text.startswith("Cats are mammals")
    assert text.endswith("I like cats")
