//! Integration: governance quorum, moderator elections, recall.

use janmat_governance::ProposalType;
use pallet_governance::{GovernancePallet, ProposalStatus};

fn seeded() -> GovernancePallet {
    let mut p = GovernancePallet::default();
    p.seed_constitution(vec![
        ("quorum.regular", "0.501"),
        ("quorum.constitutional", "0.67"),
    ], 0);
    p
}

#[test]
fn constitutional_amendment_full_flow() {
    let mut p = seeded();

    // Propose a constitutional change
    let id = {
        let prop = p.create_proposal(
            ProposalType::Constitutional,
            "Extend moderator term to 180 blocks",
            "Longer terms for stability",
            "wallet-initiator",
            100,
            200, // 200 eligible voters
        );
        prop.id.clone()
    };

    // 65% vote YES → should FAIL (need 67%)
    for i in 0..130u64 {
        p.vote_on_proposal(&id, &format!("v-{i}"), true).unwrap();
    }
    assert_eq!(p.close_proposal(&id).unwrap(), ProposalStatus::Rejected);

    // Second proposal — 70% YES → PASSES
    let id2 = {
        let prop = p.create_proposal(
            ProposalType::Constitutional,
            "Extend moderator term to 180 blocks v2",
            "d", "w", 200, 200,
        );
        prop.id.clone()
    };
    for i in 0..140u64 {
        p.vote_on_proposal(&id2, &format!("u-{i}"), true).unwrap();
    }
    assert_eq!(p.close_proposal(&id2).unwrap(), ProposalStatus::Passed);

    // Enact the rule change
    p.enact_rule("moderator.term_blocks", "180", &id2, 300);
    assert_eq!(p.get_rule("moderator.term_blocks").unwrap().value, "180");
    assert_eq!(p.get_rule("moderator.term_blocks").unwrap().previous_value, None); // was not seeded
    assert_eq!(p.constitution_versions.len(), 2); // genesis + this change
}

#[test]
fn moderator_recall_at_10_percent_opens_vote() {
    let mut p = seeded();

    // Elect a moderator
    p.elect_moderator("t/topic-1", "mod-corrupt", 10).unwrap();
    assert_eq!(p.active_moderators("t/topic-1", 10).len(), 1);

    // 9 signatures → not triggered
    for i in 0..9usize {
        let triggered = p.sign_recall("t/topic-1", "mod-corrupt", &format!("v-{i}"), 100);
        assert!(!triggered);
    }

    // 10th signature → triggers recall
    let triggered = p.sign_recall("t/topic-1", "mod-corrupt", "v-9", 100);
    assert!(triggered);

    // Simulate outcome: recall passes → remove moderator
    p.remove_moderator("t/topic-1", "mod-corrupt").unwrap();
    assert_eq!(p.active_moderators("t/topic-1", 10).len(), 0);
}

#[test]
fn citizen_referenda_auto_triggers() {
    let mut p = seeded();

    // 4 signatures → not triggered (need 5% of 100 = 5)
    for i in 0..4usize {
        assert!(!p.sign_referenda("ref:clean-water-policy", &format!("w-{i}"), 100));
    }
    // 5th → triggers
    let triggered = p.sign_referenda("ref:clean-water-policy", "w-4", 100);
    assert!(triggered);

    // Further signatures do not re-trigger
    assert!(!p.sign_referenda("ref:clean-water-policy", "w-99", 100));
}

#[test]
fn validator_set_progressive_decentralisation() {
    let mut p = seeded();

    // Pass a constitutional proposal to add validators
    let id = {
        let prop = p.create_proposal(ProposalType::Constitutional, "Add IIT validators", "d", "w", 1, 100);
        prop.id.clone()
    };
    for i in 0..67u64 {
        p.vote_on_proposal(&id, &format!("v-{i}"), true).unwrap();
    }
    p.close_proposal(&id).unwrap();

    p.add_validator(&id, "validator-IIT-Bombay", 500).unwrap();
    p.add_validator(&id, "validator-IIT-Delhi",  500).unwrap();
    p.add_validator(&id, "validator-IISc",       500).unwrap();
    assert_eq!(p.active_validator_count(), 3);

    // Pass another proposal to remove one
    let id2 = {
        let prop = p.create_proposal(ProposalType::Constitutional, "Remove IISc validator", "d", "w2", 2, 100);
        prop.id.clone()
    };
    for i in 0..67u64 {
        p.vote_on_proposal(&id2, &format!("u-{i}"), true).unwrap();
    }
    p.close_proposal(&id2).unwrap();
    p.remove_validator(&id2, "validator-IISc").unwrap();
    assert_eq!(p.active_validator_count(), 2);
}
