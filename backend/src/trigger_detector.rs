use std::sync::RwLock;

#[derive(Default)]
pub struct TriggerDetector {
    keywords: RwLock<Vec<String>>,
}

impl TriggerDetector {
    pub fn add_keyword(&self, keyword: String) {
        self.keywords.write().unwrap().push(keyword);
    }

    pub fn detect(&self, text: &str) -> Vec<String> {
        let kws = self.keywords.read().unwrap();
        kws.iter()
            .filter(|k| text.to_lowercase().contains(&k.to_lowercase()))
            .cloned()
            .collect()
    }
}
