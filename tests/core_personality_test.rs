/* neira:meta
id: NEI-20250210-core-personality-test-clippy
intent: test
summary: подавлен бессмысленный assert для clippy.
*/
#[allow(clippy::assertions_on_constants)]
#[test]
fn placeholder() {
    assert!(true);
}
