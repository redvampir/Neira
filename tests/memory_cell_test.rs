struct MemoryCell {
    store: Vec<String>,
}

impl MemoryCell {
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
    let mut cell = MemoryCell::new();
    cell.remember("data");
    assert_eq!(cell.recall(), Some(&"data".to_string()));
}
