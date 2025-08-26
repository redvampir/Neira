struct SupportHub {
    log: Vec<String>,
}

impl SupportHub {
    fn new() -> Self {
        Self { log: Vec::new() }
    }

    fn dispatch(&mut self, request: &str) {
        self.log.push(request.to_string());
    }
}

#[test]
fn records_dispatch() {
    let mut hub = SupportHub::new();
    hub.dispatch("ping");
    assert_eq!(hub.log, vec!["ping".to_string()]);
}
