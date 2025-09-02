/* neira:meta
id: NEI-20250210-code-cell-test-clippy
intent: test
summary: подавлен lint о бессмысленном assert при запуске clippy.
*/
#[allow(clippy::assertions_on_constants)]
#[test]
fn placeholder() {
    assert!(true);
}
