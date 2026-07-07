//! Integration: Merkle anchor + indexer pipeline.

use janmat_storage::{
    indexer::{InMemoryIndexStore, IndexReader, IndexerEvent},
    merkle::{MerkleAnchor, batch_votes_root, merkle_root},
};
use std::collections::HashMap;

#[test]
fn merkle_root_anchors_batch_of_votes() {
    let mut anchor = MerkleAnchor::default();
    let commits: Vec<String> = (0..100).map(|i| format!("{i:064x}")).collect();
    let record = anchor.anchor_batch(&commits, 1000);

    assert_eq!(record.commitment_count, 100);
    assert_eq!(record.batch_block, 1000);
    assert!(!record.root_hex.is_empty());
    assert_eq!(anchor.anchored_roots.len(), 1);

    // Re-computing the same batch gives the same root
    let root2 = batch_votes_root(&commits);
    let hex2: String = root2.iter().map(|b| format!("{b:02x}")).collect();
    assert_eq!(record.root_hex, hex2);
}

#[test]
fn indexer_full_vote_lifecycle_query() {
    let mut store = InMemoryIndexStore::default();

    // Ingest events in order
    store.ingest(IndexerEvent::TopicCreated {
        topic_id: "t/nep".into(),
        slug: "education-nep".into(),
        block: 100,
    });
    store.ingest(IndexerEvent::OpinionSubmitted {
        opinion_id: "o/op1".into(),
        subtopic_id: "st/rural".into(),
        creator_nullifier: "null-alice".into(),
        content_cid: "QmABC".into(),
        block: 200,
    });
    let mut tally = HashMap::new();
    tally.insert("YES".to_owned(), 73u64);
    tally.insert("NO".to_owned(), 27u64);
    store.ingest(IndexerEvent::VoteClosed {
        vote_id: "v/vote1".into(),
        tally,
        block: 9000,
    });
    store.ingest(IndexerEvent::OpinionFlagged {
        opinion_id: "o/op1".into(),
        reason_code: "review".into(),
        moderator: "mod-A".into(),
        block: 400,
    });

    // Queries
    assert_eq!(store.topics().len(), 1);
    assert_eq!(store.opinions_for_subtopic("st/rural").len(), 1);
    let tally = store.vote_tally("v/vote1").unwrap();
    assert_eq!(tally["YES"], 73);
    assert_eq!(store.moderation_log("o/op1").len(), 1);
    assert_eq!(store.event_count(), 4);
}

#[test]
fn merkle_tree_properties() {
    // Two leaves
    let leaves = vec![[1u8; 32], [2u8; 32]];
    let root = merkle_root(&leaves);
    assert_ne!(root, [0u8; 32]);

    // Order matters
    let reversed = vec![[2u8; 32], [1u8; 32]];
    assert_ne!(root, merkle_root(&reversed));

    // Adding a leaf changes the root
    let mut three_leaves = leaves.clone();
    three_leaves.push([3u8; 32]);
    assert_ne!(root, merkle_root(&three_leaves));
}
