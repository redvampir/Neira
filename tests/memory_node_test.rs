struct MemoryNode {
    store: Vec<String>,
}

impl MemoryNode {
    fn new() -> Self {
        Self { store: Vec::new() }
    }

    fn remember(&mut self, data: &str) {
        self.store.push(data.to_string());
    }

    fn recall(&self) -> Option<&String> {
        self.store.last()
    }
}

#[test]
fn remembers_and_recalls() {
    let mut node = MemoryNode::new();
    node.remember("data");
    assert_eq!(node.recall(), Some(&"data".to_string()));
}
