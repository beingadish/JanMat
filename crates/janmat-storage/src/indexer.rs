//! Chain indexer event model.
//!
//! The indexer listens to Substrate block events and writes a PostgreSQL
//! read model.  This module defines the event enum and the IndexReader trait
//! so the API layer never queries the chain directly.
//!
//! Phase 6: wire to a real `sqlx` PostgreSQL pool.
//! Phase 3–5: the `InMemoryIndexStore` mock is used in tests.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── Events emitted by pallets ─────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IndexerEvent {
    // Namespace
    TopicCreated    { topic_id: String, slug: String, block: u64 },
    SubtopicCreated { subtopic_id: String, slug: String, parent_topic_id: String, block: u64 },
    OpinionSubmitted{ opinion_id: String, subtopic_id: String, creator_nullifier: String, content_cid: String, block: u64 },
    OpinionSuppressed { opinion_id: String, block: u64 },

    // Voting
    VoteCreated     { vote_id: String, opinion_id: String, closes_at_block: u64, block: u64 },
    VoteCast        { vote_id: String, nullifier_hex: String, commitment_hex: String, seq: u64, block: u64 },
    VoteClosed      { vote_id: String, tally: HashMap<String, u64>, block: u64 },
    VoteRevealed    { vote_id: String, block: u64 },

    // Identity
    WalletRegistered { wallet: String, nullifier_seed_commitment_hex: String, block: u64 },

    // Moderation
    OpinionFlagged  { opinion_id: String, reason_code: String, moderator: String, block: u64 },
    JuryConvened    { opinion_id: String, jurors: Vec<String>, block: u64 },
    JuryVerdictFinalised { opinion_id: String, verdict: String, block: u64 },

    // Governance
    ProposalCreated { proposal_id: String, proposal_type: String, block: u64 },
    ProposalClosed  { proposal_id: String, status: String, block: u64 },
    ModeratorElected { topic_id: String, wallet: String, term_end_block: u64, block: u64 },
    ModeratorRemoved { topic_id: String, wallet: String, block: u64 },
    RecallTriggered  { topic_id: String, moderator: String, block: u64 },
    RuleEnacted      { key: String, value: String, proposal_id: String, block: u64 },

    // Merkle anchor
    MerkleRootAnchored { root_hex: String, batch_block: u64, commitment_count: usize },
}

// ── Reader trait ──────────────────────────────────────────────────────────────

pub trait IndexReader {
    fn topics(&self) -> Vec<TopicRow>;
    fn opinions_for_subtopic(&self, subtopic_id: &str) -> Vec<OpinionRow>;
    fn vote_tally(&self, vote_id: &str) -> Option<HashMap<String, u64>>;
    fn moderation_log(&self, opinion_id: &str) -> Vec<IndexerEvent>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicRow {
    pub topic_id: String,
    pub slug: String,
    pub created_at_block: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpinionRow {
    pub opinion_id: String,
    pub subtopic_id: String,
    pub content_cid: String,
    pub suppressed: bool,
    pub created_at_block: u64,
}

// ── In-memory mock ────────────────────────────────────────────────────────────

#[derive(Default)]
pub struct InMemoryIndexStore {
    pub events: Vec<IndexerEvent>,
}

impl InMemoryIndexStore {
    pub fn ingest(&mut self, event: IndexerEvent) {
        self.events.push(event);
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }
}

impl IndexReader for InMemoryIndexStore {
    fn topics(&self) -> Vec<TopicRow> {
        self.events.iter().filter_map(|e| {
            if let IndexerEvent::TopicCreated { topic_id, slug, block } = e {
                Some(TopicRow { topic_id: topic_id.clone(), slug: slug.clone(), created_at_block: *block })
            } else { None }
        }).collect()
    }

    fn opinions_for_subtopic(&self, subtopic_id: &str) -> Vec<OpinionRow> {
        self.events.iter().filter_map(|e| {
            if let IndexerEvent::OpinionSubmitted { opinion_id, subtopic_id: st, content_cid, block, .. } = e {
                if st == subtopic_id {
                    return Some(OpinionRow {
                        opinion_id: opinion_id.clone(),
                        subtopic_id: st.clone(),
                        content_cid: content_cid.clone(),
                        suppressed: false,
                        created_at_block: *block,
                    });
                }
            }
            None
        }).collect()
    }

    fn vote_tally(&self, vote_id: &str) -> Option<HashMap<String, u64>> {
        self.events.iter().find_map(|e| {
            if let IndexerEvent::VoteClosed { vote_id: vid, tally, .. } = e {
                if vid == vote_id { return Some(tally.clone()); }
            }
            None
        })
    }

    fn moderation_log(&self, opinion_id: &str) -> Vec<IndexerEvent> {
        self.events.iter().filter(|e| matches!(e,
            IndexerEvent::OpinionFlagged  { opinion_id: oid, .. } |
            IndexerEvent::JuryConvened    { opinion_id: oid, .. } |
            IndexerEvent::JuryVerdictFinalised { opinion_id: oid, .. }
            if oid == opinion_id
        )).cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ingest_and_query_topics() {
        let mut store = InMemoryIndexStore::default();
        store.ingest(IndexerEvent::TopicCreated { topic_id: "t/1".into(), slug: "nep".into(), block: 100 });
        store.ingest(IndexerEvent::TopicCreated { topic_id: "t/2".into(), slug: "farm".into(), block: 200 });
        assert_eq!(store.topics().len(), 2);
    }

    #[test]
    fn query_vote_tally_after_close() {
        let mut store = InMemoryIndexStore::default();
        let mut tally = HashMap::new();
        tally.insert("YES".to_owned(), 73u64);
        tally.insert("NO".to_owned(), 27u64);
        store.ingest(IndexerEvent::VoteClosed { vote_id: "v/1".into(), tally, block: 9000 });
        let result = store.vote_tally("v/1").unwrap();
        assert_eq!(result["YES"], 73);
    }

    #[test]
    fn moderation_log_filters_by_opinion() {
        let mut store = InMemoryIndexStore::default();
        store.ingest(IndexerEvent::OpinionFlagged { opinion_id: "o/1".into(), reason_code: "hate".into(), moderator: "mod-A".into(), block: 500 });
        store.ingest(IndexerEvent::OpinionFlagged { opinion_id: "o/2".into(), reason_code: "clone".into(), moderator: "mod-B".into(), block: 501 });
        let log = store.moderation_log("o/1");
        assert_eq!(log.len(), 1);
    }

    #[test]
    fn opinions_for_subtopic_returns_correct_subset() {
        let mut store = InMemoryIndexStore::default();
        store.ingest(IndexerEvent::OpinionSubmitted { opinion_id: "o/1".into(), subtopic_id: "st/1".into(), creator_nullifier: "n".into(), content_cid: "Qm1".into(), block: 100 });
        store.ingest(IndexerEvent::OpinionSubmitted { opinion_id: "o/2".into(), subtopic_id: "st/2".into(), creator_nullifier: "n".into(), content_cid: "Qm2".into(), block: 101 });
        assert_eq!(store.opinions_for_subtopic("st/1").len(), 1);
        assert_eq!(store.opinions_for_subtopic("st/2").len(), 1);
        assert_eq!(store.opinions_for_subtopic("st/99").len(), 0);
    }
}
