use janmat_types::{AnchoredUlid, OpinionId, SubtopicId, TopicId, VoteId};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// ── Records ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicRecord {
    pub id: TopicId,
    pub anchor: AnchoredUlid,
    pub slug: String,
    pub moderators: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtopicRecord {
    pub id: SubtopicId,
    pub anchor: AnchoredUlid,
    pub slug: String,
    pub parent_topic_id: TopicId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpinionRecord {
    pub id: OpinionId,
    pub anchor: AnchoredUlid,
    /// IPFS CID of the opinion text content
    pub content_cid: String,
    /// Blake3 hash of the raw content — chain-verified integrity
    pub content_hash: String,
    pub parent_subtopic_id: SubtopicId,
    pub creator_nullifier: String,
    /// ULID string of another opinion this is declared similar to, if any
    pub clone_of: Option<String>,
    pub suppressed: bool,
}

// ── Outcomes tracker ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OutcomeStatus {
    /// Policy passed — awaiting the 180-day outcome assessment window
    AwaitingAssessment,
    /// Assessment vote is open
    AssessmentOpen,
    /// Assessment vote closed with a result
    Assessed { result: String },
}

/// Links a policy vote's output to an outcome assessment vote.
/// Created ~180 days after the original v/ closes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutcomeRecord {
    pub output_vote_id: VoteId,
    pub assessment_vote_id: Option<VoteId>,
    pub status: OutcomeStatus,
    pub evidence_cids: Vec<String>,
    pub linked_at_block: u64,
}

// ── Errors ────────────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Eq)]
pub enum NamespaceError {
    TopicNotFound,
    SubtopicNotFound,
    OpinionNotFound,
    SlugAlreadyTaken,
}

// ── Pallet ────────────────────────────────────────────────────────────────────

#[derive(Default)]
pub struct NamespacePallet {
    topics: BTreeMap<String, TopicRecord>,
    subtopics: BTreeMap<String, SubtopicRecord>,
    opinions: BTreeMap<String, OpinionRecord>,
    pub outcomes: BTreeMap<String, OutcomeRecord>,
}

impl NamespacePallet {
    pub fn create_topic(
        &mut self,
        slug: &str,
        block_hash: &[u8],
    ) -> Result<TopicRecord, NamespaceError> {
        let anchor = AnchoredUlid::new(block_hash);
        let id = TopicId(anchor.id);
        let key = id.to_string();
        let record = TopicRecord {
            id,
            anchor,
            slug: slug.to_owned(),
            moderators: Vec::new(),
        };
        self.topics.insert(key, record.clone());
        Ok(record)
    }

    pub fn create_subtopic(
        &mut self,
        slug: &str,
        parent_topic_id: TopicId,
        block_hash: &[u8],
    ) -> Result<SubtopicRecord, NamespaceError> {
        if !self.topics.contains_key(&parent_topic_id.to_string()) {
            return Err(NamespaceError::TopicNotFound);
        }
        let anchor = AnchoredUlid::new(block_hash);
        let id = SubtopicId(anchor.id);
        let record = SubtopicRecord {
            id,
            anchor,
            slug: slug.to_owned(),
            parent_topic_id,
        };
        self.subtopics.insert(record.id.to_string(), record.clone());
        Ok(record)
    }

    /// Opinions are immutable once stored — content_cid and content_hash are final.
    pub fn submit_opinion(
        &mut self,
        parent_subtopic_id: SubtopicId,
        content_cid: String,
        content_hash: String,
        creator_nullifier: String,
        clone_of: Option<String>,
        block_hash: &[u8],
    ) -> Result<OpinionRecord, NamespaceError> {
        if !self.subtopics.contains_key(&parent_subtopic_id.to_string()) {
            return Err(NamespaceError::SubtopicNotFound);
        }
        let anchor = AnchoredUlid::new(block_hash);
        let id = OpinionId(anchor.id);
        let record = OpinionRecord {
            id,
            anchor,
            content_cid,
            content_hash,
            parent_subtopic_id,
            creator_nullifier,
            clone_of,
            suppressed: false,
        };
        self.opinions.insert(record.id.to_string(), record.clone());
        Ok(record)
    }

    pub fn suppress_opinion(&mut self, opinion_id: &str) -> Result<(), NamespaceError> {
        self.opinions
            .get_mut(opinion_id)
            .map(|op| op.suppressed = true)
            .ok_or(NamespaceError::OpinionNotFound)
    }

    // ── Outcomes tracker ──────────────────────────────────────────────────────

    /// Register a closed policy vote for outcome tracking.
    /// Called ~180 blocks after the vote closes (governance schedule).
    pub fn register_outcome(
        &mut self,
        output_vote_id: VoteId,
        at_block: u64,
    ) -> OutcomeRecord {
        let record = OutcomeRecord {
            output_vote_id: output_vote_id.clone(),
            assessment_vote_id: None,
            status: OutcomeStatus::AwaitingAssessment,
            evidence_cids: Vec::new(),
            linked_at_block: at_block,
        };
        self.outcomes.insert(output_vote_id.to_string(), record.clone());
        record
    }

    /// Open the assessment vote for an outcome.
    pub fn open_assessment(
        &mut self,
        output_vote_id: &str,
        assessment_vote_id: VoteId,
    ) -> Result<(), NamespaceError> {
        let rec = self.outcomes
            .get_mut(output_vote_id)
            .ok_or(NamespaceError::OpinionNotFound)?;
        rec.assessment_vote_id = Some(assessment_vote_id);
        rec.status = OutcomeStatus::AssessmentOpen;
        Ok(())
    }

    /// Add evidence IPFS CID to an outcome record.
    pub fn add_evidence(&mut self, output_vote_id: &str, cid: String) -> Result<(), NamespaceError> {
        self.outcomes
            .get_mut(output_vote_id)
            .ok_or(NamespaceError::OpinionNotFound)?
            .evidence_cids
            .push(cid);
        Ok(())
    }

    /// Finalise the outcome assessment with a result string (e.g. "61% say NO CHANGE").
    pub fn finalise_outcome(&mut self, output_vote_id: &str, result: String) -> Result<(), NamespaceError> {
        let rec = self.outcomes
            .get_mut(output_vote_id)
            .ok_or(NamespaceError::OpinionNotFound)?;
        rec.status = OutcomeStatus::Assessed { result };
        Ok(())
    }

    pub fn topic_count(&self) -> usize { self.topics.len() }
    pub fn opinion_count(&self) -> usize { self.opinions.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_pallet() -> NamespacePallet {
        NamespacePallet::default()
    }

    #[test]
    fn create_topic_subtopic_opinion_chain() {
        let mut p = make_pallet();
        let topic = p.create_topic("education-nep-2026", b"block1").unwrap();
        let st = p
            .create_subtopic("nep-rural", topic.id.clone(), b"block2")
            .unwrap();
        let op = p
            .submit_opinion(
                st.id.clone(),
                "Qmcid".to_owned(),
                "blake3hash".to_owned(),
                "nullifier-abc".to_owned(),
                None,
                b"block3",
            )
            .unwrap();
        assert_eq!(p.opinion_count(), 1);
        assert!(!op.suppressed);
    }

    #[test]
    fn subtopic_requires_valid_parent() {
        let mut p = make_pallet();
        let fake_topic_id = TopicId::new();
        let err = p.create_subtopic("orphan", fake_topic_id, b"b").unwrap_err();
        assert_eq!(err, NamespaceError::TopicNotFound);
    }

    #[test]
    fn suppress_marks_opinion_not_deletes() {
        let mut p = make_pallet();
        let topic = p.create_topic("test-topic", b"b1").unwrap();
        let st = p.create_subtopic("st", topic.id.clone(), b"b2").unwrap();
        let op = p
            .submit_opinion(st.id.clone(), "cid".to_owned(), "h".to_owned(), "n".to_owned(), None, b"b3")
            .unwrap();
        p.suppress_opinion(&op.id.to_string()).unwrap();
        // opinion still in map — only status changes
        assert_eq!(p.opinion_count(), 1);
    }

    #[test]
    fn outcomes_lifecycle() {
        use janmat_types::VoteId;
        let mut p = make_pallet();
        let vote_id = VoteId::new();
        let key = vote_id.to_string();
        p.register_outcome(vote_id.clone(), 1000);
        assert_eq!(p.outcomes[&key].status, OutcomeStatus::AwaitingAssessment);

        let assessment_id = VoteId::new();
        p.open_assessment(&key, assessment_id).unwrap();
        assert_eq!(p.outcomes[&key].status, OutcomeStatus::AssessmentOpen);

        p.add_evidence(&key, "QmEvidence123".to_owned()).unwrap();
        assert_eq!(p.outcomes[&key].evidence_cids.len(), 1);

        p.finalise_outcome(&key, "61% say NO CHANGE".to_owned()).unwrap();
        assert!(matches!(p.outcomes[&key].status, OutcomeStatus::Assessed { .. }));
    }
}
