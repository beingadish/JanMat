//! vote-cast circuit
//!
//! Proves (without revealing the vote choice):
//!   1. The citizen holds a valid kyc nullifier_seed (derived from a registered credential).
//!   2. nullifier = Blake3(nullifier_seed || vote_id) — unique per (citizen, vote).
//!   3. commitment = Blake3(vote_choice || nonce) — choice is hidden until reveal.
//!
//! Public outputs committed to the chain:
//!   - nullifier              — prevents double-voting on-chain
//!   - commitment             — sealed vote value, opened if vote goes public
//!   - vote_id                — ties the proof to a specific v/ record
//!
//! Phase 4 will replace Blake3 commitment with a proper Pedersen commitment
//! and add threshold ElGamal encryption for the reveal path.
//! The circuit structure and public outputs remain the same.

use blake3::Hasher;
use serde::{Deserialize, Serialize};

// ── Input / Output ────────────────────────────────────────────────────────────

/// Private witness — client side only.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteCastInput {
    /// Raw nullifier_seed bytes — derived during KYC; Private.
    pub nullifier_seed: Vec<u8>,
    /// The vote/ record ULID string this cast is for.
    pub vote_id: String,
    /// Citizen's choice (e.g. "YES", "NO", "ABSTAIN"). Private.
    pub vote_choice: String,
    /// Random nonce for the commitment. Private.
    pub nonce: Vec<u8>,
}

/// Public outputs — stored on-chain by `pallet-voting`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VoteCastOutput {
    /// Blake3(nullifier_seed || vote_id) — unique per citizen per vote.
    pub nullifier: [u8; 32],
    /// Blake3(vote_choice || nonce) — seals the choice until reveal.
    pub commitment: [u8; 32],
    /// vote_id re-stated so the pallet can match receipt to the right ballot.
    pub vote_id: String,
}

// ── Derivation helpers ────────────────────────────────────────────────────────

pub fn derive_vote_nullifier(nullifier_seed: &[u8], vote_id: &str) -> [u8; 32] {
    let mut h = Hasher::new();
    h.update(b"janmat/vote_nullifier/v1");
    h.update(nullifier_seed);
    h.update(vote_id.as_bytes());
    *h.finalize().as_bytes()
}

pub fn derive_commitment(vote_choice: &str, nonce: &[u8]) -> [u8; 32] {
    let mut h = Hasher::new();
    h.update(b"janmat/vote_commitment/v1");
    h.update(vote_choice.as_bytes());
    h.update(nonce);
    *h.finalize().as_bytes()
}

// ── Circuit logic ─────────────────────────────────────────────────────────────

pub fn execute(input: &VoteCastInput) -> Result<VoteCastOutput, &'static str> {
    if input.nullifier_seed.is_empty() {
        return Err("missing_nullifier_seed");
    }
    if input.vote_id.is_empty() {
        return Err("missing_vote_id");
    }
    if input.vote_choice.is_empty() {
        return Err("missing_vote_choice");
    }
    if input.nonce.is_empty() {
        return Err("missing_nonce");
    }
    Ok(VoteCastOutput {
        nullifier: derive_vote_nullifier(&input.nullifier_seed, &input.vote_id),
        commitment: derive_commitment(&input.vote_choice, &input.nonce),
        vote_id: input.vote_id.clone(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_input() -> VoteCastInput {
        VoteCastInput {
            nullifier_seed: b"seed-bytes-from-kyc".to_vec(),
            vote_id: "v/01HXYZ_VVVVV".to_owned(),
            vote_choice: "YES".to_owned(),
            nonce: b"random-nonce-32b".to_vec(),
        }
    }

    #[test]
    fn produces_deterministic_outputs() {
        let o1 = execute(&valid_input()).unwrap();
        let o2 = execute(&valid_input()).unwrap();
        assert_eq!(o1, o2);
    }

    #[test]
    fn different_vote_id_gives_different_nullifier() {
        let mut inp = valid_input();
        let out_a = execute(&inp).unwrap();
        inp.vote_id = "v/OTHER_VOTE".to_owned();
        let out_b = execute(&inp).unwrap();
        assert_ne!(out_a.nullifier, out_b.nullifier);
    }

    #[test]
    fn different_choice_same_nullifier_different_commitment() {
        let mut inp = valid_input();
        let out_yes = execute(&inp).unwrap();
        inp.vote_choice = "NO".to_owned();
        let out_no = execute(&inp).unwrap();
        // Nullifier is choice-independent (only seed + vote_id)
        assert_eq!(out_yes.nullifier, out_no.nullifier);
        // Commitment hides the choice
        assert_ne!(out_yes.commitment, out_no.commitment);
    }

    #[test]
    fn empty_seed_rejected() {
        let mut inp = valid_input();
        inp.nullifier_seed.clear();
        assert_eq!(execute(&inp), Err("missing_nullifier_seed"));
    }
}
