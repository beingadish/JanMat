//! End-to-end integration: full citizen lifecycle.
//!
//! Scenario: Alice registers via ZK KYC, submits an opinion on a subtopic,
//! creates a vote, casts YES, a second citizen (Bob) casts NO, tally is closed,
//! results are verified.  Alice changes her vote before close (re-vote test).

use janmat_identity::{
    kyc_provider::{KycProvider, MockKycProvider},
    nullifier::derive_nullifier_seed,
};
use janmat_types::namespace::OpinionId;
use janmat_zk::{MockVerifier, circuits::vote_cast::VoteCastInput, prover::MockProver};
use pallet_identity::IdentityPallet;
use pallet_namespace::NamespacePallet;
use pallet_voting::VotingPallet;

fn register(identity: &mut IdentityPallet, wallet: &str, subject: &str) {
    let cred = MockKycProvider.issue_credential(subject);
    let receipt = MockProver.prove_kyc(&cred.to_kyc_input()).unwrap();
    identity
        .register_wallet(wallet, &receipt, 1, &MockVerifier)
        .unwrap();
}

fn vote_receipt(vote_id: &str, choice: &str, subject: &str) -> janmat_zk::ZkReceipt {
    let cred = MockKycProvider.issue_credential(subject);
    let seed = derive_nullifier_seed(&cred.credential_bytes, &cred.citizen_salt).to_vec();
    MockProver
        .prove_vote(&VoteCastInput {
            nullifier_seed: seed,
            vote_id: vote_id.to_owned(),
            vote_choice: choice.to_owned(),
            nonce: b"nonce-32-bytes-padded-here-ok!".to_vec(),
        })
        .unwrap()
}

#[test]
fn full_citizen_vote_lifecycle() {
    let mut identity = IdentityPallet::default();
    let mut namespace = NamespacePallet::default();
    let mut voting = VotingPallet::default();

    // ── Identity ─────────────────────────────────────────────────────────────
    register(&mut identity, "wallet-alice", "citizen-alice");
    register(&mut identity, "wallet-bob", "citizen-bob");
    assert_eq!(identity.citizen_count(), 2);

    // ── Namespace: topic → subtopic → opinion ─────────────────────────────────
    let topic = namespace
        .create_topic("education-nep-2026", b"block-1")
        .unwrap();
    let st = namespace
        .create_subtopic("nep-rural", topic.id.clone(), b"block-2")
        .unwrap();
    let op = namespace
        .submit_opinion(
            st.id.clone(),
            "QmCID123".into(),
            "blake3hash".into(),
            "nullifier-alice".into(),
            None,
            b"block-3",
        )
        .unwrap();
    assert_eq!(namespace.opinion_count(), 1);

    // ── Vote: create ──────────────────────────────────────────────────────────
    let vote = voting.create_vote(op.id.clone(), vec!["YES".into(), "NO".into()], 9000);
    let vid = vote.id.to_string();

    // ── Alice casts YES ───────────────────────────────────────────────────────
    let r_alice_yes = vote_receipt(&vid, "YES", "citizen-alice");
    voting.cast_vote(&vid, &r_alice_yes, &MockVerifier).unwrap();

    // ── Alice changes mind → NO (re-vote — coercion resistance) ──────────────
    let r_alice_no = vote_receipt(&vid, "NO", "citizen-alice");
    voting.cast_vote(&vid, &r_alice_no, &MockVerifier).unwrap();
    // Still only 1 entry (re-vote replaces)
    assert_eq!(voting.get_vote(&vid).unwrap().casts.len(), 1);

    // ── Bob casts YES ─────────────────────────────────────────────────────────
    let r_bob = vote_receipt(&vid, "YES", "citizen-bob");
    voting.cast_vote(&vid, &r_bob, &MockVerifier).unwrap();
    assert_eq!(voting.get_vote(&vid).unwrap().casts.len(), 2);

    // ── Close tally ───────────────────────────────────────────────────────────
    let tally = voting.close_tally(&vid).unwrap();
    // Alice = NO, Bob = YES — 1 each (by commitment, not by choice name in Phase 4)
    assert_eq!(tally.values().sum::<u64>(), 2);
}

#[test]
fn double_registration_prevented() {
    let mut identity = IdentityPallet::default();
    register(&mut identity, "wallet-alice", "citizen-alice");
    // Same citizen, different wallet
    let cred = MockKycProvider.issue_credential("citizen-alice");
    let receipt = MockProver.prove_kyc(&cred.to_kyc_input()).unwrap();
    let err = identity.register_wallet("wallet-alice-2", &receipt, 2, &MockVerifier);
    assert!(err.is_err());
}

#[test]
fn vote_after_close_rejected() {
    let mut voting = VotingPallet::default();
    let vote = voting.create_vote(OpinionId::new(), vec!["YES".into()], 100);
    let vid = vote.id.to_string();
    voting.close_tally(&vid).unwrap();
    let receipt = vote_receipt(&vid, "YES", "citizen-charlie");
    assert!(voting.cast_vote(&vid, &receipt, &MockVerifier).is_err());
}

#[test]
fn reveal_requires_closed_vote() {
    let mut voting = VotingPallet::default();
    let vote = voting.create_vote(OpinionId::new(), vec!["YES".into()], 100);
    let vid = vote.id.to_string();
    // Reveal before close → error
    assert!(voting.reveal_toggle(&vid).is_err());
    // Close then reveal
    voting.close_tally(&vid).unwrap();
    voting.reveal_toggle(&vid).unwrap();
    // Second reveal → error
    assert!(voting.reveal_toggle(&vid).is_err());
}
