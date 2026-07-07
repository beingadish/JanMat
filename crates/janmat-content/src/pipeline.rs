use crate::{
    blocklist::BlocklistEngine,
    clone_detector::CloneDetector,
    harm_filter::{HarmDisposition, HarmFilter, HarmReport},
};

// ── Request ───────────────────────────────────────────────────────────────────

pub struct SubmitRequest<'a> {
    pub subtopic_id: &'a str,
    pub text: &'a str,
}

// ── Report ────────────────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct CheckReport {
    pub verdict: Verdict,
    pub harm: HarmReport,
    pub clone_candidates: Vec<String>,
    /// Phrase that triggered a blocklist block (if any)
    pub blocked_by_phrase: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Verdict {
    /// Submission may proceed immediately.
    Clean,
    /// Submission is hard-rejected; reason in `blocked_by_phrase` or `harm`.
    Blocked,
    /// Submission is held for moderator + jury review.
    NeedsReview,
    /// Submission may proceed but submitter must acknowledge near-duplicate(s).
    CloneWarning,
}

// ── Pipeline ──────────────────────────────────────────────────────────────────

pub struct ContentPipeline {
    pub blocklist: BlocklistEngine,
    harm: HarmFilter,
    pub clone: CloneDetector,
}

impl ContentPipeline {
    pub fn new(blocklist: BlocklistEngine) -> Self {
        Self {
            blocklist,
            harm: HarmFilter,
            clone: CloneDetector::default(),
        }
    }

    /// Main entry point. Runs: blocklist → harm filter → clone detector.
    pub fn check(&self, req: &SubmitRequest<'_>) -> CheckReport {
        // 1. Blocklist (exact phrase match — fast, no ML cost)
        if let Some(phrase) = self.blocklist.contains_blocked_phrase(req.text) {
            let harm = self.harm.score(req.text);
            return CheckReport {
                verdict: Verdict::Blocked,
                harm,
                clone_candidates: vec![],
                blocked_by_phrase: Some(phrase.to_owned()),
            };
        }

        // 2. Harm filter (ML scoring)
        let harm = self.harm.score(req.text);
        let verdict = match harm.disposition() {
            HarmDisposition::Block => Verdict::Blocked,
            HarmDisposition::Review => Verdict::NeedsReview,
            HarmDisposition::Pass => {
                // 3. Clone detection (only if not already flagged)
                let clones = self.clone.scan(req.subtopic_id, req.text);
                if !clones.is_empty() {
                    let clone_candidates: Vec<String> =
                        clones.into_iter().map(|c| c.candidate_id).collect();
                    return CheckReport {
                        verdict: Verdict::CloneWarning,
                        harm,
                        clone_candidates,
                        blocked_by_phrase: None,
                    };
                }
                Verdict::Clean
            }
        };

        CheckReport {
            verdict,
            harm,
            clone_candidates: vec![],
            blocked_by_phrase: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pipeline(phrases: &[&str]) -> ContentPipeline {
        ContentPipeline::new(BlocklistEngine::with_phrases(
            phrases.iter().map(|s| s.to_string()).collect(),
        ))
    }

    #[test]
    fn clean_text_passes() {
        let p = pipeline(&[]);
        let r = p.check(&SubmitRequest {
            subtopic_id: "st/1",
            text: "NEP reform is important",
        });
        assert_eq!(r.verdict, Verdict::Clean);
    }

    #[test]
    fn blocked_phrase_stops_pipeline() {
        let p = pipeline(&["banned-word"]);
        let r = p.check(&SubmitRequest {
            subtopic_id: "st/1",
            text: "contains banned-word here",
        });
        assert_eq!(r.verdict, Verdict::Blocked);
        assert!(r.blocked_by_phrase.is_some());
    }

    #[test]
    fn harm_coordination_keyword_blocks() {
        let p = pipeline(&[]);
        let r = p.check(&SubmitRequest {
            subtopic_id: "st/1",
            text: "[coordinate-attack] [coordinate-attack] [coordinate-attack] [coordinate-attack]",
        });
        assert_eq!(r.verdict, Verdict::Blocked);
        assert!(r.blocked_by_phrase.is_none()); // blocked by harm, not blocklist
    }

    #[test]
    fn clone_warning_returned_for_near_duplicate() {
        let mut p = pipeline(&[]);
        let original = "nep policy reform rural india students need help";
        p.clone.index_opinion("st/1", "o/original", original);
        let r = p.check(&SubmitRequest {
            subtopic_id: "st/1",
            text: original,
        });
        assert_eq!(r.verdict, Verdict::CloneWarning);
        assert!(!r.clone_candidates.is_empty());
    }
}
