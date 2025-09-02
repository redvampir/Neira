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
    let mut cell = ActionCell::new();
    cell.execute();
    assert!(cell.executed);
}
