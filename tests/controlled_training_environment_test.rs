/* neira:meta
id: NEI-20250210-controlled-training-clippy
intent: test
summary: подавлен бессмысленный assert для clippy.
*/
#[allow(clippy::assertions_on_constants)]
#[test]
fn placeholder() {
    assert!(true);
}
