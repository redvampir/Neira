use neira::greet;

#[test]
fn greet_returns_hello() {
    assert_eq!(greet(), "Hello, Neira!");
}
