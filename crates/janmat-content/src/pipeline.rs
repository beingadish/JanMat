use crate::{blocklist::BlocklistEngine, clone_detector::CloneDetector, harm_filter::HarmFilter};

#[derive(Debug)]
pub enum ContentCheckResult {
    Clean,
    Blocked { reason: String },
    NeedsReview { harm_score: u8 },
    CloneWarning { candidates: Vec<String> },
}

pub struct ContentPipeline {
    blocklist: BlocklistEngine,
    harm: HarmFilter,
    clone: CloneDetector,
}

impl ContentPipeline {
    pub fn new(blocklist: BlocklistEngine) -> Self {
        Self {
            blocklist,
            harm: HarmFilter,
            clone: CloneDetector,
        }
    }

    pub fn check(&self, text: &str) -> ContentCheckResult {
        if self.blocklist.contains_blocked_phrase(text) {
            return ContentCheckResult::Blocked {
                reason: "blocked_phrase".to_owned(),
            };
        }
        let score = self.harm.score(text).0;
        if score >= 80 {
            return ContentCheckResult::Blocked {
                reason: format!("harm_score:{score}"),
            };
        }
        if score >= 40 {
            return ContentCheckResult::NeedsReview { harm_score: score };
        }
        let clones = self.clone.scan(text);
        if !clones.is_empty() {
            let ids: Vec<String> = clones.into_iter().map(|c| c.candidate_id).collect();
            return ContentCheckResult::CloneWarning { candidates: ids };
        }
        ContentCheckResult::Clean
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pipeline_with_blocklist(phrases: &[&str]) -> ContentPipeline {
        ContentPipeline::new(BlocklistEngine::with_phrases(
            phrases.iter().map(|s| s.to_string()).collect(),
        ))
    }

    #[test]
    fn clean_text_passes() {
        let p = pipeline_with_blocklist(&[]);
        assert!(matches!(p.check("NEP reform is important"), ContentCheckResult::Clean));
    }

    #[test]
    fn blocked_phrase_stops_pipeline() {
        let p = pipeline_with_blocklist(&["banned-word"]);
        assert!(matches!(
            p.check("This contains banned-word here"),
            ContentCheckResult::Blocked { .. }
        ));
    }
}
