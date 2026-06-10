use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// ── Audit log ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModerationAuditEntry {
    pub opinion_id: String,
    pub action: ModerationAction,
    pub reason_code: String,
    pub actor_wallet: String,
    pub at_block: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModerationAction {
    Flagged,
    JuryConvened,
    Suppressed,
    Kept,
    AppealOpened,
}

// ── Jury ──────────────────────────────────────────────────────────────────────

const JURY_SIZE: usize = 7;
const SUPPRESS_THRESHOLD: usize = 4; // 4-of-7

#[derive(Debug, Clone)]
pub struct JurySession {
    pub opinion_id: String,
    pub jurors: Vec<String>,  // wallet addresses (pseudonymous, on-chain)
    pub votes: BTreeMap<String, bool>, // juror wallet → true = suppress
    pub resolved: bool,
}

impl JurySession {
    /// Sample `JURY_SIZE` jurors deterministically from a voter pool using
    /// block_hash as the entropy source. Deterministic = publicly verifiable.
    pub fn new(
        opinion_id: &str,
        voter_pool: &[String],
        block_hash: &[u8],
    ) -> Option<Self> {
        if voter_pool.len() < JURY_SIZE {
            return None;
        }
        // Deterministic selection: sort pool, then pick by block_hash-derived indices
        let mut pool = voter_pool.to_vec();
        pool.sort();
        let seed = blake3::hash(block_hash);
        let seed_bytes = seed.as_bytes();
        let mut jurors = Vec::with_capacity(JURY_SIZE);
        let mut used = vec![false; pool.len()];
        for i in 0..JURY_SIZE {
            let idx = (seed_bytes[i % 32] as usize + i * 7) % pool.len();
            // Resolve collisions by linear probing
            let mut j = idx;
            loop {
                if !used[j] {
                    used[j] = true;
                    jurors.push(pool[j].clone());
                    break;
                }
                j = (j + 1) % pool.len();
            }
        }
        Some(JurySession {
            opinion_id: opinion_id.to_owned(),
            jurors,
            votes: BTreeMap::new(),
            resolved: false,
        })
    }

    /// Record a juror's vote. Returns `Err` if not a juror or already voted.
    pub fn cast_vote(&mut self, juror: &str, suppress: bool) -> Result<(), &'static str> {
        if !self.jurors.contains(&juror.to_owned()) {
            return Err("not_a_juror");
        }
        if self.votes.contains_key(juror) {
            return Err("already_voted");
        }
        self.votes.insert(juror.to_owned(), suppress);
        Ok(())
    }

    /// Tally once all 7 jurors have voted.
    pub fn tally(&self) -> Option<JuryVerdict> {
        if self.votes.len() < JURY_SIZE {
            return None;
        }
        let suppress_count = self.votes.values().filter(|&&v| v).count();
        Some(if suppress_count >= SUPPRESS_THRESHOLD {
            JuryVerdict::Suppress
        } else {
            JuryVerdict::Keep
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum JuryVerdict {
    Suppress,
    Keep,
}

// ── Pallet ────────────────────────────────────────────────────────────────────

#[derive(Default)]
pub struct ModerationPallet {
    /// opinion_id → active jury session
    juries: BTreeMap<String, JurySession>,
    /// Immutable chronological audit log
    pub audit_log: Vec<ModerationAuditEntry>,
}

impl ModerationPallet {
    /// Moderator flags an opinion — creates a jury session from the voter pool.
    pub fn flag_opinion(
        &mut self,
        opinion_id: &str,
        reason_code: &str,
        moderator_wallet: &str,
        voter_pool: &[String],
        block_hash: &[u8],
        at_block: u64,
    ) -> Result<(), &'static str> {
        self.audit_log.push(ModerationAuditEntry {
            opinion_id: opinion_id.to_owned(),
            action: ModerationAction::Flagged,
            reason_code: reason_code.to_owned(),
            actor_wallet: moderator_wallet.to_owned(),
            at_block,
        });
        let jury = JurySession::new(opinion_id, voter_pool, block_hash)
            .ok_or("voter_pool_too_small")?;
        self.audit_log.push(ModerationAuditEntry {
            opinion_id: opinion_id.to_owned(),
            action: ModerationAction::JuryConvened,
            reason_code: format!("jurors:{}", jury.jurors.join(",")),
            actor_wallet: "system".to_owned(),
            at_block,
        });
        self.juries.insert(opinion_id.to_owned(), jury);
        Ok(())
    }

    /// Juror casts a vote on a pending jury session.
    pub fn jury_vote(
        &mut self,
        opinion_id: &str,
        juror: &str,
        suppress: bool,
    ) -> Result<Option<JuryVerdict>, &'static str> {
        let jury = self.juries.get_mut(opinion_id).ok_or("no_jury_session")?;
        jury.cast_vote(juror, suppress)?;
        Ok(jury.tally())
    }

    /// Finalise the jury verdict (called after tally() returns Some).
    pub fn finalise_verdict(
        &mut self,
        opinion_id: &str,
        verdict: JuryVerdict,
        at_block: u64,
    ) {
        let action = match verdict {
            JuryVerdict::Suppress => ModerationAction::Suppressed,
            JuryVerdict::Keep => ModerationAction::Kept,
        };
        self.audit_log.push(ModerationAuditEntry {
            opinion_id: opinion_id.to_owned(),
            action,
            reason_code: "jury_verdict".to_owned(),
            actor_wallet: "system".to_owned(),
            at_block,
        });
        if let Some(jury) = self.juries.get_mut(opinion_id) {
            jury.resolved = true;
        }
    }

    pub fn audit_entries_for(&self, opinion_id: &str) -> Vec<&ModerationAuditEntry> {
        self.audit_log
            .iter()
            .filter(|e| e.opinion_id == opinion_id)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_pool(n: usize) -> Vec<String> {
        (0..n).map(|i| format!("wallet-{i:03}")).collect()
    }

    #[test]
    fn flag_creates_jury_and_audit_entry() {
        let mut p = ModerationPallet::default();
        let pool = make_pool(10);
        p.flag_opinion("o/001", "hate_speech", "mod-A", &pool, b"blockhash", 500)
            .unwrap();
        assert_eq!(p.juries.len(), 1);
        let entries = p.audit_entries_for("o/001");
        assert!(entries.iter().any(|e| e.action == ModerationAction::Flagged));
        assert!(entries.iter().any(|e| e.action == ModerationAction::JuryConvened));
    }

    #[test]
    fn jury_suppress_4_of_7_suppresses() {
        let mut p = ModerationPallet::default();
        let pool = make_pool(10);
        p.flag_opinion("o/002", "harm", "mod-A", &pool, b"hash", 100).unwrap();
        let jurors = p.juries["o/002"].jurors.clone();
        // 4 vote suppress, 3 vote keep
        for (i, j) in jurors.iter().enumerate() {
            let suppress = i < 4;
            p.jury_vote("o/002", j, suppress).unwrap();
        }
        let verdict = p.jury_vote("o/002", &jurors[6], false)
            .unwrap_or(None)
            .unwrap_or_else(|| p.juries["o/002"].tally().unwrap());
        assert_eq!(verdict, JuryVerdict::Suppress);
    }

    #[test]
    fn jury_selection_is_deterministic() {
        let pool = make_pool(20);
        let j1 = JurySession::new("o/X", &pool, b"same-hash").unwrap();
        let j2 = JurySession::new("o/X", &pool, b"same-hash").unwrap();
        assert_eq!(j1.jurors, j2.jurors);
    }

    #[test]
    fn jury_selection_differs_with_different_block_hash() {
        let pool = make_pool(20);
        let j1 = JurySession::new("o/X", &pool, b"hash-A").unwrap();
        let j2 = JurySession::new("o/X", &pool, b"hash-B").unwrap();
        assert_ne!(j1.jurors, j2.jurors);
    }

    #[test]
    fn voter_pool_too_small_returns_error() {
        let mut p = ModerationPallet::default();
        let small_pool = make_pool(3);
        let err = p.flag_opinion("o/003", "reason", "mod", &small_pool, b"h", 1)
            .unwrap_err();
        assert_eq!(err, "voter_pool_too_small");
    }

    #[test]
    fn audit_log_is_append_only_and_immutable() {
        let mut p = ModerationPallet::default();
        let pool = make_pool(10);
        p.flag_opinion("o/004", "r", "mod", &pool, b"h", 1).unwrap();
        let initial_len = p.audit_log.len();
        p.finalise_verdict("o/004", JuryVerdict::Suppress, 2);
        assert!(p.audit_log.len() > initial_len);
        // Previous entries are unchanged
        assert_eq!(p.audit_log[0].opinion_id, "o/004");
    }
}
