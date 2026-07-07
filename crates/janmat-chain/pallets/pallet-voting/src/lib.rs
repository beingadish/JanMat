//! pallet-voting — Phase 4
//!
//! Key invariants enforced:
//!   • Each citizen may only hold ONE active commitment per vote at close time.
//!     Re-voting is allowed: the latest valid cast supersedes the previous one
//!     (coercion resistance — a coercer cannot verify what the final vote was).
//!   • reveal_toggle is strictly one-way: Anonymous → Public.
//!   • enc_attr (ElGamal-encrypted wallet||choice||nonce) is stored alongside
//!     every commitment so the reveal path can attribute votes without trusting
//!     the chain operator.  Phase 4 uses a Blake3 stub encryption; Phase 6 will
//!     replace with threshold ElGamal keyed to the validator set.

use janmat_types::{
    OpinionId, VoteId,
    vote::{VoteStatus, VoteVisibility},
};
use janmat_zk::{CIRCUIT_VOTE_CAST, ZkReceipt, ZkVerifier, circuits::vote_cast::VoteCastOutput};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

// ── Records ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CastRecord {
    /// Hex-encoded nullifier from the ZK proof public outputs
    pub nullifier_hex: String,
    /// Hex-encoded commitment from the ZK proof public outputs
    pub commitment_hex: String,
    /// Stub-encrypted attribution bundle for the reveal path.
    /// Format: Blake3("enc" || nullifier_hex || commitment_hex)
    /// Phase 6: replace with threshold ElGamal ciphertext.
    pub enc_attr: String,
    /// Cast sequence number — later casts for the same nullifier supersede earlier ones
    pub seq: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributedVote {
    pub nullifier_hex: String,
    pub commitment_hex: String,
    /// Decrypted attribution: e.g. "wallet-A:YES"  (stub reveal)
    pub attributed_to: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteRecord {
    pub id: VoteId,
    pub opinion_id: OpinionId,
    pub options: Vec<String>,
    pub visibility: VoteVisibility,
    pub status: VoteStatus,
    pub closes_at_block: u64,
    /// Latest cast per nullifier (nullifier_hex → CastRecord)
    pub casts: BTreeMap<String, CastRecord>,
    /// Populated after close
    pub tally: Option<HashMap<String, u64>>,
    /// Populated after reveal_toggle
    pub attributed: Option<Vec<AttributedVote>>,
    /// Global cast sequence counter
    cast_seq: u64,
}

// ── Errors ────────────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Eq)]
pub enum VoteError {
    VoteNotFound,
    InvalidProof,
    InvalidPublicOutputs,
    VoteMismatch,
    VoteAlreadyClosed,
    AlreadyPublic,
    NotYetClosed,
}

// ── Pallet ────────────────────────────────────────────────────────────────────

#[derive(Default)]
pub struct VotingPallet {
    votes: BTreeMap<String, VoteRecord>,
}

impl VotingPallet {
    pub fn create_vote(
        &mut self,
        opinion_id: OpinionId,
        options: Vec<String>,
        closes_at_block: u64,
    ) -> VoteRecord {
        let id = VoteId::new();
        let record = VoteRecord {
            id,
            opinion_id,
            options,
            visibility: VoteVisibility::Anonymous,
            status: VoteStatus::Open,
            closes_at_block,
            casts: BTreeMap::new(),
            tally: None,
            attributed: None,
            cast_seq: 0,
        };
        self.votes.insert(record.id.to_string(), record.clone());
        record
    }

    /// Register a ballot under a caller-supplied id/key.
    ///
    /// Used when the API or indexer re-hydrates a vote that was created by a
    /// separate request/extrinsic, so a later cast can be matched to it without
    /// replaying the original `create_vote`. (In Phase 4 the pallet is rebuilt
    /// per request; Phase 5 backs this with shared indexer state.)
    pub fn open_vote(&mut self, vote_id: &str, options: Vec<String>, closes_at_block: u64) {
        let record = VoteRecord {
            id: VoteId::new(),
            opinion_id: OpinionId::new(),
            options,
            visibility: VoteVisibility::Anonymous,
            status: VoteStatus::Open,
            closes_at_block,
            casts: BTreeMap::new(),
            tally: None,
            attributed: None,
            cast_seq: 0,
        };
        self.votes.insert(vote_id.to_string(), record);
    }

    /// Cast (or re-cast) a vote backed by a ZK proof receipt.
    ///
    /// Re-voting is explicitly allowed: the latest valid cast for a given
    /// nullifier replaces the previous one.  Only casts present at `close_tally`
    /// time count.  This prevents coercers from verifying a specific choice was
    /// locked in — the citizen can change their mind until polls close.
    pub fn cast_vote<V: ZkVerifier>(
        &mut self,
        vote_id: &str,
        receipt: &ZkReceipt,
        verifier: &V,
    ) -> Result<CastRecord, VoteError> {
        // 1. Verify the ZK receipt
        if !verifier.verify(receipt, CIRCUIT_VOTE_CAST) {
            return Err(VoteError::InvalidProof);
        }
        // 2. Deserialise public outputs
        let output: VoteCastOutput = serde_json::from_slice(&receipt.public_outputs)
            .map_err(|_| VoteError::InvalidPublicOutputs)?;
        // 3. Ensure the proof is for THIS vote
        if output.vote_id != vote_id {
            return Err(VoteError::VoteMismatch);
        }
        let record = self.votes.get_mut(vote_id).ok_or(VoteError::VoteNotFound)?;
        if record.status == VoteStatus::Closed {
            return Err(VoteError::VoteAlreadyClosed);
        }

        let nullifier_hex = hex_bytes(&output.nullifier);
        let commitment_hex = hex_bytes(&output.commitment);

        // 4. Build enc_attr stub: Blake3("enc" || nullifier || commitment)
        let enc_attr = {
            let mut payload = b"enc:".to_vec();
            payload.extend_from_slice(nullifier_hex.as_bytes());
            payload.extend_from_slice(commitment_hex.as_bytes());
            hex_bytes(blake3::hash(&payload).as_bytes())
        };

        record.cast_seq += 1;
        let cast = CastRecord {
            nullifier_hex: nullifier_hex.clone(),
            commitment_hex,
            enc_attr,
            seq: record.cast_seq,
        };
        // Overwrite any previous cast for this nullifier (re-vote)
        record.casts.insert(nullifier_hex, cast.clone());
        Ok(cast)
    }

    /// Close the tally at the designated block. Counts the latest commitment
    /// per nullifier.  After this point no new casts are accepted.
    pub fn close_tally(&mut self, vote_id: &str) -> Result<HashMap<String, u64>, VoteError> {
        let record = self.votes.get_mut(vote_id).ok_or(VoteError::VoteNotFound)?;
        record.status = VoteStatus::Closed;
        let mut tally: HashMap<String, u64> = HashMap::new();
        for cast in record.casts.values() {
            *tally.entry(cast.commitment_hex.clone()).or_insert(0) += 1;
        }
        record.tally = Some(tally.clone());
        Ok(tally)
    }

    /// Reveal toggle — strictly one-way: Anonymous → Public.
    ///
    /// On reveal: decrypt each enc_attr stub and populate `attributed`.
    /// Phase 4 stub: attribution reconstructed from enc_attr metadata.
    /// Phase 6: threshold ElGamal decryption with 7-of-11 validator cooperation.
    pub fn reveal_toggle(&mut self, vote_id: &str) -> Result<(), VoteError> {
        let record = self.votes.get_mut(vote_id).ok_or(VoteError::VoteNotFound)?;
        if record.visibility == VoteVisibility::Public {
            return Err(VoteError::AlreadyPublic);
        }
        if record.status != VoteStatus::Closed {
            return Err(VoteError::NotYetClosed);
        }
        record.visibility = VoteVisibility::Public;
        // Stub reveal: attribute commitment to nullifier (Phase 6: real decrypt)
        let attributed: Vec<AttributedVote> = record
            .casts
            .values()
            .map(|cast| AttributedVote {
                nullifier_hex: cast.nullifier_hex.clone(),
                commitment_hex: cast.commitment_hex.clone(),
                attributed_to: format!("nullifier:{}", &cast.nullifier_hex[..8]),
            })
            .collect();
        record.attributed = Some(attributed);
        Ok(())
    }

    pub fn get_vote(&self, vote_id: &str) -> Option<&VoteRecord> {
        self.votes.get(vote_id)
    }

    pub fn vote_count(&self) -> usize {
        self.votes.len()
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn hex_bytes(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use janmat_identity::nullifier::derive_nullifier_seed;
    use janmat_types::namespace::OpinionId;
    use janmat_zk::{MockVerifier, circuits::vote_cast::VoteCastInput, prover::MockProver};

    fn make_seed() -> Vec<u8> {
        derive_nullifier_seed(b"mock-credential", b"mock-salt").to_vec()
    }

    fn prove_vote(vote_id: &str, choice: &str, seed: &[u8]) -> ZkReceipt {
        MockProver
            .prove_vote(&VoteCastInput {
                nullifier_seed: seed.to_vec(),
                vote_id: vote_id.to_owned(),
                vote_choice: choice.to_owned(),
                nonce: b"nonce-32-bytes-padded-padd-here!".to_vec(),
            })
            .unwrap()
    }

    fn make_vote(p: &mut VotingPallet) -> String {
        let rec = p.create_vote(OpinionId::new(), vec!["YES".into(), "NO".into()], 9000);
        rec.id.to_string()
    }

    #[test]
    fn full_zk_vote_cast_flow() {
        let mut p = VotingPallet::default();
        let vid = make_vote(&mut p);
        let seed = make_seed();
        let receipt = prove_vote(&vid, "YES", &seed);
        p.cast_vote(&vid, &receipt, &MockVerifier).unwrap();
        assert_eq!(p.votes[&vid].casts.len(), 1);
    }

    #[test]
    fn double_cast_same_nullifier_is_revote_not_error() {
        // Re-voting is allowed — later cast supersedes earlier one (coercion resistance)
        let mut p = VotingPallet::default();
        let vid = make_vote(&mut p);
        let seed = make_seed();
        let r1 = prove_vote(&vid, "YES", &seed);
        let r2 = prove_vote(&vid, "NO", &seed);
        p.cast_vote(&vid, &r1, &MockVerifier).unwrap();
        p.cast_vote(&vid, &r2, &MockVerifier).unwrap();
        // Still only 1 entry per nullifier — the latest one
        assert_eq!(p.votes[&vid].casts.len(), 1);
        let cast = p.votes[&vid].casts.values().next().unwrap();
        assert!(cast.seq == 2); // seq incremented on each cast
    }

    #[test]
    fn different_citizens_different_nullifiers() {
        let mut p = VotingPallet::default();
        let vid = make_vote(&mut p);
        let seed_a = derive_nullifier_seed(b"cred-A", b"salt-A").to_vec();
        let seed_b = derive_nullifier_seed(b"cred-B", b"salt-B").to_vec();
        p.cast_vote(&vid, &prove_vote(&vid, "YES", &seed_a), &MockVerifier)
            .unwrap();
        p.cast_vote(&vid, &prove_vote(&vid, "NO", &seed_b), &MockVerifier)
            .unwrap();
        assert_eq!(p.votes[&vid].casts.len(), 2);
    }

    #[test]
    fn vote_after_close_rejected() {
        let mut p = VotingPallet::default();
        let vid = make_vote(&mut p);
        p.close_tally(&vid).unwrap();
        let seed = make_seed();
        let err = p
            .cast_vote(&vid, &prove_vote(&vid, "YES", &seed), &MockVerifier)
            .unwrap_err();
        assert_eq!(err, VoteError::VoteAlreadyClosed);
    }

    #[test]
    fn wrong_vote_id_in_proof_rejected() {
        let mut p = VotingPallet::default();
        let vid = make_vote(&mut p);
        let seed = make_seed();
        // Prove for a different vote_id
        let receipt = prove_vote("v/WRONG_VOTE", "YES", &seed);
        let err = p.cast_vote(&vid, &receipt, &MockVerifier).unwrap_err();
        assert_eq!(err, VoteError::VoteMismatch);
    }

    #[test]
    fn tally_counts_only_latest_cast_per_citizen() {
        let mut p = VotingPallet::default();
        let vid = make_vote(&mut p);
        let seed = make_seed();
        // Citizen votes YES then changes to NO
        p.cast_vote(&vid, &prove_vote(&vid, "YES", &seed), &MockVerifier)
            .unwrap();
        p.cast_vote(&vid, &prove_vote(&vid, "NO", &seed), &MockVerifier)
            .unwrap();
        let tally = p.close_tally(&vid).unwrap();
        // Only 1 commitment total (the re-vote replaced the first)
        assert_eq!(tally.values().sum::<u64>(), 1);
    }

    #[test]
    fn reveal_toggle_is_one_way_and_requires_closed_vote() {
        let mut p = VotingPallet::default();
        let vid = make_vote(&mut p);
        // Cannot reveal an open vote
        let err = p.reveal_toggle(&vid).unwrap_err();
        assert_eq!(err, VoteError::NotYetClosed);
        // Close then reveal
        p.close_tally(&vid).unwrap();
        p.reveal_toggle(&vid).unwrap();
        // Second reveal attempt
        let err = p.reveal_toggle(&vid).unwrap_err();
        assert_eq!(err, VoteError::AlreadyPublic);
    }

    #[test]
    fn reveal_populates_attributed_records() {
        let mut p = VotingPallet::default();
        let vid = make_vote(&mut p);
        let seed = make_seed();
        p.cast_vote(&vid, &prove_vote(&vid, "YES", &seed), &MockVerifier)
            .unwrap();
        p.close_tally(&vid).unwrap();
        p.reveal_toggle(&vid).unwrap();
        let attributed = p.votes[&vid].attributed.as_ref().unwrap();
        assert_eq!(attributed.len(), 1);
        assert!(!attributed[0].attributed_to.is_empty());
    }

    #[test]
    fn invalid_receipt_rejected() {
        let mut p = VotingPallet::default();
        let vid = make_vote(&mut p);
        let bad = ZkReceipt {
            circuit_id: "wrong".to_owned(),
            proof_bytes: vec![1],
            public_outputs: vec![],
        };
        let err = p.cast_vote(&vid, &bad, &MockVerifier).unwrap_err();
        assert_eq!(err, VoteError::InvalidProof);
    }
}
