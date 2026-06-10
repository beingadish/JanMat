//! janmat-governance — shared governance primitives
//!
//! Used by pallet-governance and the API layer.

use serde::{Deserialize, Serialize};

// ── Quorum constants ──────────────────────────────────────────────────────────

pub const REGULAR_QUORUM: f64       = 0.501; // simple majority
pub const CONSTITUTIONAL_QUORUM: f64 = 0.67;  // two-thirds supermajority
pub const RECALL_PETITION_RATIO: f64 = 0.10;  // 10% of eligible voters
pub const REFERENDA_SUPPORT_RATIO: f64 = 0.05; // 5% to open a citizen referenda

// ── Quorum math ───────────────────────────────────────────────────────────────

pub fn met_quorum(yes_votes: u64, total_votes: u64, constitutional: bool) -> bool {
    if total_votes == 0 {
        return false;
    }
    let target = if constitutional { CONSTITUTIONAL_QUORUM } else { REGULAR_QUORUM };
    (yes_votes as f64 / total_votes as f64) >= target
}

pub fn recall_threshold(eligible_voters: u64) -> u64 {
    ((eligible_voters as f64 * RECALL_PETITION_RATIO).ceil()) as u64
}

pub fn referenda_threshold(eligible_voters: u64) -> u64 {
    ((eligible_voters as f64 * REFERENDA_SUPPORT_RATIO).ceil()) as u64
}

// ── Proposal types ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProposalType {
    /// Regular policy vote — 50%+1 quorum, standard duration
    Regular,
    /// Constitutional amendment — 67% supermajority, extended duration
    Constitutional,
    /// Moderator election within a topic — regular quorum
    ModeratorElection { topic_id: String },
    /// Moderator recall — triggered automatically at 10% petition threshold
    ModeratorRecall { topic_id: String, moderator_wallet: String },
    /// Citizen-initiated referenda — auto-opened at 5% signatures
    CitizenReferenda { initiator_wallet: String },
}

impl ProposalType {
    pub fn is_constitutional(&self) -> bool {
        matches!(self, ProposalType::Constitutional)
    }

    pub fn required_quorum(&self) -> f64 {
        if self.is_constitutional() {
            CONSTITUTIONAL_QUORUM
        } else {
            REGULAR_QUORUM
        }
    }
}

// ── Constitution rule ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionRule {
    pub key: String,
    pub value: String,
    /// Block at which this rule version became active
    pub enacted_at_block: u64,
    /// The proposal ULID that enacted this version
    pub enacted_by_proposal: String,
    /// Previous value (None = genesis rule)
    pub previous_value: Option<String>,
}

// ── Constitution version entry ────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionVersion {
    pub version: u32,
    pub rules_snapshot: Vec<ConstitutionRule>,
    pub enacted_at_block: u64,
    pub enacted_by_proposal: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_quorum_50_plus_1() {
        assert!(!met_quorum(50, 100, false));
        assert!(met_quorum(51, 100, false));
    }

    #[test]
    fn constitutional_requires_two_thirds() {
        assert!(!met_quorum(66, 100, true));
        assert!(met_quorum(67, 100, true));
    }

    #[test]
    fn recall_threshold_is_ten_percent_ceiling() {
        assert_eq!(recall_threshold(100), 10);
        assert_eq!(recall_threshold(101), 11); // ceiling
        assert_eq!(recall_threshold(0), 0);
    }

    #[test]
    fn referenda_threshold_is_five_percent_ceiling() {
        assert_eq!(referenda_threshold(100), 5);
        assert_eq!(referenda_threshold(201), 11);
    }

    #[test]
    fn proposal_type_quorum() {
        assert_eq!(ProposalType::Regular.required_quorum(), REGULAR_QUORUM);
        assert_eq!(ProposalType::Constitutional.required_quorum(), CONSTITUTIONAL_QUORUM);
        assert_eq!(
            ProposalType::ModeratorElection { topic_id: "t/1".into() }.required_quorum(),
            REGULAR_QUORUM
        );
    }
}
