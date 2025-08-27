use std::sync::RwLock;

pub struct TriggerDetector {
    keywords: RwLock<Vec<String>>,
}

impl Default for TriggerDetector {
    fn default() -> Self {
        let defaults = vec![
            "биология".to_string(),
            "программирование".to_string(),
            "rust".to_string(),
            "математика".to_string(),
            "нейросети".to_string(),
        ];
        Self {
            keywords: RwLock::new(defaults),
        }
    }
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
