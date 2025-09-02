struct ActionCell {
    executed: bool,
}

impl ActionCell {
    fn new() -> Self {
        Self { executed: false }
    }

    fn execute(&mut self) {
        self.executed = true;
    }
}

#[test]
fn action_executes() {
    let mut node = ActionCell::new();
    node.execute();
    assert!(node.executed);
}
