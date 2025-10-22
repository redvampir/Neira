/* neira:meta
id: NEI-20270420-trybuild-tests
intent: test
summary: Компиляционные проверки DigestiveSettings через trybuild.
*/
use trybuild::TestCases;

#[test]
fn digestive_settings_compile_checks() {
    let t = TestCases::new();
    t.pass("tests/trybuild/digestive_settings_ok.rs");
    t.compile_fail("tests/trybuild/digestive_settings_wrong_type.rs");
    t.compile_fail("tests/trybuild/digestive_settings_unknown_field.rs");
}
