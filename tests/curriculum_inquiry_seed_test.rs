/* neira:meta
id: NEI-20251101-curriculum-seed-tests
intent: test
summary: Тесты: сортировка/лимит build_inquiry_seed и env-предел RUSSIAN_CURRICULUM_MAX_WORDS.
*/
use backend::training::curriculum::{RussianLiteracyCurriculum, INQUIRY_SEED_LIMIT};

#[test]
fn seed_respects_limit_and_is_non_decreasing_by_level() {
    std::env::remove_var("RUSSIAN_CURRICULUM_MAX_WORDS");
    // Load default curriculum from repo files
    let cur = RussianLiteracyCurriculum::load_default().expect("curriculum should load");
    let seed = cur.build_inquiry_seed();
    assert!(seed.len() <= INQUIRY_SEED_LIMIT);

    // Non-decreasing by level
    for w in seed.windows(2) {
        let a = &w[0];
        let b = &w[1];
        assert!(a.level <= b.level, "levels must be non-decreasing");
    }
}

#[test]
fn load_respects_env_words_limit() {
    // Force a very small limit so validation fails
    std::env::set_var("RUSSIAN_CURRICULUM_MAX_WORDS", "1");
    let res = RussianLiteracyCurriculum::load_default();
    assert!(
        res.is_err(),
        "validation should fail when words exceed env limit"
    );
    std::env::remove_var("RUSSIAN_CURRICULUM_MAX_WORDS");
}
