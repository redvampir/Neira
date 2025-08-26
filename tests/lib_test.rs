use neira::greet;

#[test]
fn greet_returns_hello() {
    assert_eq!(greet(), "Hello, Neira!");
}

#[test]
fn greet_has_correct_length() {
    assert_eq!(greet().len(), 13);
}
