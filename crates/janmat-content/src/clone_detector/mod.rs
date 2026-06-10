//! Clone detector — TF-IDF cosine similarity across opinions in the same subtopic.
//!
//! Phase 3: pure-Rust TF-IDF with an in-memory per-subtopic index.
//! Phase 6: swap to a dense embedding model (IndicBERT via candle) for
//! cross-lingual matching; the `CloneDetector` trait interface stays the same.

use std::collections::HashMap;

pub const CLONE_THRESHOLD: f32 = 0.85;

#[derive(Debug, Clone)]
pub struct CloneMatch {
    pub candidate_id: String,
    pub score: f32,
}

// ── TF-IDF helpers ────────────────────────────────────────────────────────────

fn tokenise(text: &str) -> Vec<String> {
    text.split_whitespace()
        .map(|w| w.to_lowercase().trim_matches(|c: char| !c.is_alphanumeric()).to_owned())
        .filter(|w| !w.is_empty())
        .collect()
}

fn term_freq(tokens: &[String]) -> HashMap<String, f32> {
    let mut tf: HashMap<String, f32> = HashMap::new();
    for t in tokens {
        *tf.entry(t.clone()).or_insert(0.0) += 1.0;
    }
    let total = tokens.len() as f32;
    tf.values_mut().for_each(|v| *v /= total);
    tf
}

fn cosine_sim(a: &HashMap<String, f32>, b: &HashMap<String, f32>) -> f32 {
    let dot: f32 = a.iter().map(|(k, va)| b.get(k).unwrap_or(&0.0) * va).sum();
    let mag_a: f32 = a.values().map(|v| v * v).sum::<f32>().sqrt();
    let mag_b: f32 = b.values().map(|v| v * v).sum::<f32>().sqrt();
    if mag_a == 0.0 || mag_b == 0.0 {
        return 0.0;
    }
    dot / (mag_a * mag_b)
}

// ── Index ─────────────────────────────────────────────────────────────────────

struct IndexedOpinion {
    id: String,
    tf: HashMap<String, f32>,
}

#[derive(Default)]
pub struct CloneDetector {
    /// subtopic_id → list of indexed opinions
    index: HashMap<String, Vec<IndexedOpinion>>,
}

impl CloneDetector {
    /// Index a new opinion so future submissions can be compared against it.
    pub fn index_opinion(&mut self, subtopic_id: &str, opinion_id: &str, text: &str) {
        let tokens = tokenise(text);
        let tf = term_freq(&tokens);
        self.index
            .entry(subtopic_id.to_owned())
            .or_default()
            .push(IndexedOpinion {
                id: opinion_id.to_owned(),
                tf,
            });
    }

    /// Scan a new text against all indexed opinions in the same subtopic.
    /// Returns matches above `CLONE_THRESHOLD`, sorted descending by score.
    pub fn scan(&self, subtopic_id: &str, text: &str) -> Vec<CloneMatch> {
        let tokens = tokenise(text);
        let tf = term_freq(&tokens);
        let Some(opinions) = self.index.get(subtopic_id) else {
            return Vec::new();
        };
        let mut matches: Vec<CloneMatch> = opinions
            .iter()
            .map(|op| CloneMatch {
                candidate_id: op.id.clone(),
                score: cosine_sim(&tf, &op.tf),
            })
            .filter(|m| m.score >= CLONE_THRESHOLD)
            .collect();
        matches.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        matches
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_text_is_detected_as_clone() {
        let mut d = CloneDetector::default();
        let text = "The NEP policy needs urgent reform for rural students across India";
        d.index_opinion("st/1", "o/orig", text);
        let hits = d.scan("st/1", text);
        assert!(!hits.is_empty());
        assert!(hits[0].score > 0.99);
    }

    #[test]
    fn different_text_not_flagged() {
        let mut d = CloneDetector::default();
        d.index_opinion("st/1", "o/orig", "NEP policy reform for rural India");
        let hits = d.scan("st/1", "Farmers need better irrigation subsidies");
        assert!(hits.is_empty());
    }

    #[test]
    fn no_index_for_subtopic_returns_empty() {
        let d = CloneDetector::default();
        let hits = d.scan("st/unknown", "some opinion text");
        assert!(hits.is_empty());
    }

    #[test]
    fn slightly_reworded_text_crosses_threshold() {
        let mut d = CloneDetector::default();
        d.index_opinion("st/1", "o/orig", "nep policy reform rural india students need help");
        // Near-identical reordering — should still be ≥ 0.85
        let hits = d.scan("st/1", "nep policy reform rural india students need help today");
        assert!(!hits.is_empty());
    }
}
