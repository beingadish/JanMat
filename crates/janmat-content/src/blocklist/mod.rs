use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone)]
pub struct BlocklistEntry {
    pub phrase: String,
    pub added_at_block: u64,
    /// On-chain vote/proposal that authorized this entry
    pub governance_ref: String,
}

#[derive(Debug, Clone)]
pub struct BlocklistAuditEntry {
    pub action: BlocklistAction,
    pub phrase: String,
    pub at_block: u64,
    pub governance_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlocklistAction {
    Added,
    Removed,
}

#[derive(Default)]
pub struct BlocklistEngine {
    phrases: BTreeSet<String>,
    entries: BTreeMap<String, BlocklistEntry>,
    pub audit_log: Vec<BlocklistAuditEntry>,
}

impl BlocklistEngine {
    /// Construct with pre-seeded phrases (genesis blocklist, no governance ref needed).
    pub fn with_phrases(phrases: Vec<String>) -> Self {
        let mut engine = Self::default();
        for p in phrases {
            engine.phrases.insert(p.to_lowercase());
        }
        engine
    }

    /// Add a phrase, authorized by an on-chain governance ref (vote ULID).
    pub fn add_phrase(&mut self, phrase: &str, at_block: u64, governance_ref: &str) -> bool {
        let normalised = phrase.to_lowercase();
        if self.phrases.contains(&normalised) {
            return false; // already present
        }
        self.phrases.insert(normalised.clone());
        self.entries.insert(
            normalised.clone(),
            BlocklistEntry {
                phrase: normalised.clone(),
                added_at_block: at_block,
                governance_ref: governance_ref.to_owned(),
            },
        );
        self.audit_log.push(BlocklistAuditEntry {
            action: BlocklistAction::Added,
            phrase: normalised,
            at_block,
            governance_ref: governance_ref.to_owned(),
        });
        true
    }

    /// Remove a phrase, authorized by an on-chain governance ref.
    pub fn remove_phrase(&mut self, phrase: &str, at_block: u64, governance_ref: &str) -> bool {
        let normalised = phrase.to_lowercase();
        if !self.phrases.remove(&normalised) {
            return false; // not present
        }
        self.entries.remove(&normalised);
        self.audit_log.push(BlocklistAuditEntry {
            action: BlocklistAction::Removed,
            phrase: normalised,
            at_block,
            governance_ref: governance_ref.to_owned(),
        });
        true
    }

    /// Case-insensitive substring match against all blocked phrases.
    pub fn contains_blocked_phrase(&self, text: &str) -> Option<&str> {
        let lower = text.to_lowercase();
        self.phrases
            .iter()
            .find(|p| lower.contains(p.as_str()))
            .map(|p| p.as_str())
    }

    pub fn phrase_count(&self) -> usize {
        self.phrases.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_blocked_phrase_case_insensitive() {
        let engine = BlocklistEngine::with_phrases(vec!["kill all".to_owned()]);
        assert!(
            engine
                .contains_blocked_phrase("You should KILL ALL of them")
                .is_some()
        );
    }

    #[test]
    fn clean_text_passes() {
        let engine = BlocklistEngine::with_phrases(vec!["banned".to_owned()]);
        assert!(
            engine
                .contains_blocked_phrase("This is fine content")
                .is_none()
        );
    }

    #[test]
    fn add_remove_creates_audit_trail() {
        let mut engine = BlocklistEngine::default();
        engine.add_phrase("test-phrase", 100, "v/GOV001");
        engine.remove_phrase("test-phrase", 200, "v/GOV002");
        assert_eq!(engine.phrase_count(), 0);
        assert_eq!(engine.audit_log.len(), 2);
        assert_eq!(engine.audit_log[0].action, BlocklistAction::Added);
        assert_eq!(engine.audit_log[1].action, BlocklistAction::Removed);
    }

    #[test]
    fn duplicate_add_is_idempotent() {
        let mut engine = BlocklistEngine::default();
        engine.add_phrase("dup", 1, "v/1");
        let added = engine.add_phrase("dup", 2, "v/2");
        assert!(!added);
        assert_eq!(engine.audit_log.len(), 1);
    }
}
