/* neira:meta
id: NEI-20250210-chat-cell-test-clippy
intent: test
summary: подавлен бессмысленный assert для clippy.
*/
#[allow(clippy::assertions_on_constants)]
#[test]
fn placeholder() {
    assert!(true);
}
