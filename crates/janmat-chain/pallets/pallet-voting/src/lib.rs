use janmat_types::{
    vote::{VoteStatus, VoteVisibility},
    OpinionId, VoteId,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap};

// ── Records ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteRecord {
    pub id: VoteId,
    pub opinion_id: OpinionId,
    pub options: Vec<String>,
    pub visibility: VoteVisibility,
    pub status: VoteStatus,
    pub closes_at_block: u64,
    /// Stores commitment strings for each cast vote (opaque in anonymous mode)
    pub commitments: Vec<String>,
    /// Tally populated after close
    pub tally: Option<HashMap<String, u64>>,
}

// ── Errors ────────────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Eq)]
pub enum VoteError {
    VoteNotFound,
    NullifierAlreadyUsed,
    VoteAlreadyClosed,
    AlreadyPublic,
    InvalidOption,
}

// ── Pallet ────────────────────────────────────────────────────────────────────

#[derive(Default)]
pub struct VotingPallet {
    votes: BTreeMap<String, VoteRecord>,
    /// Tracks used nullifiers per vote: (vote_id, nullifier) pairs
    nullifiers: BTreeSet<(String, String)>,
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
            commitments: Vec::new(),
            tally: None,
        };
        self.votes.insert(record.id.to_string(), record.clone());
        record
    }

    /// Cast a vote. In Phase 1 the `choice` is stored as a plain commitment
    /// placeholder; Phase 4 will replace with Pedersen commitments + ZK proof.
    pub fn cast_vote(
        &mut self,
        vote_id: &str,
        nullifier: &str,
        commitment: &str,
    ) -> Result<(), VoteError> {
        let key = (vote_id.to_owned(), nullifier.to_owned());
        if self.nullifiers.contains(&key) {
            return Err(VoteError::NullifierAlreadyUsed);
        }
        let record = self
            .votes
            .get_mut(vote_id)
            .ok_or(VoteError::VoteNotFound)?;
        if record.status == VoteStatus::Closed {
            return Err(VoteError::VoteAlreadyClosed);
        }
        self.nullifiers.insert(key);
        record.commitments.push(commitment.to_owned());
        Ok(())
    }

    /// Reveal toggle is strictly one-way: Anonymous → Public. Cannot be reversed.
    pub fn reveal_toggle(&mut self, vote_id: &str) -> Result<(), VoteError> {
        let record = self
            .votes
            .get_mut(vote_id)
            .ok_or(VoteError::VoteNotFound)?;
        if record.visibility == VoteVisibility::Public {
            return Err(VoteError::AlreadyPublic);
        }
        record.visibility = VoteVisibility::Public;
        Ok(())
    }

    /// Close tally at the designated block. Counts commitments naively in Phase 1.
    pub fn close_tally(&mut self, vote_id: &str) -> Result<(), VoteError> {
        let record = self
            .votes
            .get_mut(vote_id)
            .ok_or(VoteError::VoteNotFound)?;
        record.status = VoteStatus::Closed;
        // Phase 1: count raw commitment strings as stand-in tallying
        let mut tally: HashMap<String, u64> = HashMap::new();
        for c in &record.commitments {
            *tally.entry(c.clone()).or_insert(0) += 1;
        }
        record.tally = Some(tally);
        Ok(())
    }

    pub fn vote_count(&self) -> usize {
        self.votes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_vote(p: &mut VotingPallet) -> String {
        let op_id = OpinionId::new();
        let rec = p.create_vote(op_id, vec!["YES".into(), "NO".into()], 9000);
        rec.id.to_string()
    }

    #[test]
    fn cast_vote_records_commitment() {
        let mut p = VotingPallet::default();
        let vid = dummy_vote(&mut p);
        p.cast_vote(&vid, "null-1", "commitment-YES").unwrap();
        assert_eq!(p.votes[&vid].commitments.len(), 1);
    }

    #[test]
    fn double_vote_rejected_by_nullifier() {
        let mut p = VotingPallet::default();
        let vid = dummy_vote(&mut p);
        p.cast_vote(&vid, "null-1", "c1").unwrap();
        let err = p.cast_vote(&vid, "null-1", "c2").unwrap_err();
        assert_eq!(err, VoteError::NullifierAlreadyUsed);
    }

    #[test]
    fn reveal_toggle_is_one_way() {
        let mut p = VotingPallet::default();
        let vid = dummy_vote(&mut p);
        p.reveal_toggle(&vid).unwrap();
        let err = p.reveal_toggle(&vid).unwrap_err();
        assert_eq!(err, VoteError::AlreadyPublic);
    }

    #[test]
    fn close_tally_populates_results() {
        let mut p = VotingPallet::default();
        let vid = dummy_vote(&mut p);
        p.cast_vote(&vid, "n1", "YES").unwrap();
        p.cast_vote(&vid, "n2", "YES").unwrap();
        p.cast_vote(&vid, "n3", "NO").unwrap();
        p.close_tally(&vid).unwrap();
        let tally = p.votes[&vid].tally.as_ref().unwrap();
        assert_eq!(tally["YES"], 2);
        assert_eq!(tally["NO"], 1);
    }
}
