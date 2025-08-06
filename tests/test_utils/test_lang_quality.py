from src.utils.lang_quality import detect_language, quality_score


def test_detect_language_ru_and_en() -> None:
    assert detect_language("Hello world") == "en"
    assert detect_language("Привет мир") == "ru"


def test_quality_score_range() -> None:
    good = "This is a clean sentence."
    bad = "!!! $$$ ???"
    assert quality_score(good) > 0.8
    assert quality_score(bad) < 0.3
