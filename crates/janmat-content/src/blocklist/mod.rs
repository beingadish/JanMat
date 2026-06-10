#[derive(Default)]
pub struct BlocklistEngine {
    banned_phrases: Vec<String>,
}

impl BlocklistEngine {
    pub fn with_phrases(phrases: Vec<String>) -> Self {
        Self {
            banned_phrases: phrases,
        }
    }

    pub fn contains_blocked_phrase(&self, text: &str) -> bool {
        self.banned_phrases.iter().any(|p| text.contains(p))
    }
}
