struct ActionNode {
    executed: bool,
}

impl ActionNode {
    fn new() -> Self {
        Self { executed: false }
    }

    fn execute(&mut self) {
        self.executed = true;
    }
}

#[test]
fn action_executes() {
    let mut node = ActionNode::new();
    node.execute();
    assert!(node.executed);
}
