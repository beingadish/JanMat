//! Harm filter — scores submission text for harmful intent.
//!
//! Production: embeds an ONNX model via `candle` fine-tuned on Indian civic
//! context.  The integration point is `HarmModel::score_raw()` — swap the
//! stub for a real `candle` inference call without changing any other code.
//!
//! Phase 3: keyword heuristic that produces reproducible scores so the full
//! pipeline and moderation flow can be exercised in tests.

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HarmScore(pub u8); // 0–100

#[derive(Debug, Clone)]
pub struct HarmReport {
    pub overall: HarmScore,
    pub hate_speech: HarmScore,
    pub violence: HarmScore,
    /// Intent to coordinate harmful action (the critical category)
    pub coordination: HarmScore,
}

impl HarmReport {
    pub fn disposition(&self) -> HarmDisposition {
        match self.overall.0 {
            0..=39 => HarmDisposition::Pass,
            40..=79 => HarmDisposition::Review,
            _ => HarmDisposition::Block,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HarmDisposition {
    Pass,
    Review,
    Block,
}

// ── Model stub (replace inner body with candle inference) ────────────────────

struct HarmModel;

impl HarmModel {
    /// Score raw text → 0..=100 for a single category label.
    /// Phase 3: occurrence-count heuristic. Phase 6: ONNX via candle.
    /// Each keyword occurrence adds 50 pts (1 hit = Review, 2+ = Block), capped at 100.
    fn score_raw(text: &str, category_keywords: &[&str]) -> u8 {
        let lower = text.to_lowercase();
        let hits: usize = category_keywords
            .iter()
            .map(|kw| lower.matches(*kw).count())
            .sum();
        ((hits as u32 * 50).min(100)) as u8
    }
}

// ── Public filter ─────────────────────────────────────────────────────────────

pub struct HarmFilter;

impl HarmFilter {
    const HATE_KEYWORDS: &'static [&'static str] = &["[test-hate]", "[slur]", "[incite-hate]"];
    const VIOLENCE_KEYWORDS: &'static [&'static str] = &["[test-violence]", "[attack]", "[bomb]"];
    const COORD_KEYWORDS: &'static [&'static str] =
        &["[test-coord]", "[meet-at]", "[coordinate-attack]"];

    pub fn score(&self, text: &str) -> HarmReport {
        let hate = HarmScore(HarmModel::score_raw(text, Self::HATE_KEYWORDS));
        let violence = HarmScore(HarmModel::score_raw(text, Self::VIOLENCE_KEYWORDS));
        let coord = HarmScore(HarmModel::score_raw(text, Self::COORD_KEYWORDS));
        let overall = HarmScore(hate.0.max(violence.0).max(coord.0));
        HarmReport {
            overall,
            hate_speech: hate,
            violence,
            coordination: coord,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_civic_text_passes() {
        let f = HarmFilter;
        let r = f.score("We should reform the NEP policy for rural schools.");
        assert_eq!(r.disposition(), HarmDisposition::Pass);
    }

    #[test]
    fn coordination_keyword_triggers_block() {
        let f = HarmFilter;
        let r = f.score("Everyone [coordinate-attack] at the location [coordinate-attack] [coordinate-attack] [coordinate-attack]");
        assert_eq!(r.disposition(), HarmDisposition::Block);
    }

    #[test]
    fn single_hate_keyword_triggers_review() {
        let f = HarmFilter;
        let r = f.score("This policy is [incite-hate] unfair.");
        assert_eq!(r.disposition(), HarmDisposition::Review);
    }

    #[test]
    fn topic_about_harm_not_blocked() {
        // Discussing rape law reform — should pass (topic ≠ coordinating harm)
        let f = HarmFilter;
        let r = f.score("The rape law reform bill must be strengthened to protect survivors.");
        assert_eq!(r.disposition(), HarmDisposition::Pass);
    }
}
