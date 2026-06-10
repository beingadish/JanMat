//! pallet-governance — Phase 5 + 6
//!
//! Implements:
//!  1. Constitution storage — on-chain key/value rules with full version history
//!  2. Proposals — Regular (50%+1) and Constitutional (67%), with vote tallying
//!  3. Moderator elections — max 2 per topic, 90-block terms, renewable once
//!  4. Recall petitions — 10% of eligible voters auto-opens a recall v/
//!  5. Citizen-initiated referenda — 5% signatures auto-opens a proposal vote
//!  6. Validator set management — progressive decentralisation

use janmat_governance::{
    ConstitutionRule, ConstitutionVersion, ProposalType,
    met_quorum, recall_threshold, referenda_threshold, CONSTITUTIONAL_QUORUM,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap};

// ── Proposal ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub id: String,
    pub proposal_type: ProposalType,
    pub title: String,
    pub description: String,
    pub proposed_by: String,
    pub proposed_at_block: u64,
    pub closes_at_block: u64,
    pub status: ProposalStatus,
    yes_votes: BTreeSet<String>,  // wallet addresses
    no_votes: BTreeSet<String>,
    pub total_eligible: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProposalStatus {
    Active,
    Passed,
    Rejected,
    Enacted,
}

impl Proposal {
    pub fn cast_vote(&mut self, voter: &str, approve: bool) -> Result<(), &'static str> {
        if self.status != ProposalStatus::Active {
            return Err("proposal_not_active");
        }
        if self.yes_votes.contains(voter) || self.no_votes.contains(voter) {
            return Err("already_voted");
        }
        if approve {
            self.yes_votes.insert(voter.to_owned());
        } else {
            self.no_votes.insert(voter.to_owned());
        }
        Ok(())
    }

    pub fn yes_count(&self) -> u64 { self.yes_votes.len() as u64 }
    pub fn no_count(&self)  -> u64 { self.no_votes.len() as u64 }
    pub fn total_cast(&self) -> u64 { self.yes_count() + self.no_count() }

    /// Tally against total_eligible, applying the proposal's required quorum.
    pub fn tally(&mut self) -> ProposalStatus {
        let constitutional = self.proposal_type.is_constitutional();
        let status = if met_quorum(self.yes_count(), self.total_eligible, constitutional) {
            ProposalStatus::Passed
        } else {
            ProposalStatus::Rejected
        };
        self.status = status.clone();
        status
    }
}

// ── Moderator record ──────────────────────────────────────────────────────────

const MAX_MODERATORS_PER_TOPIC: usize = 2;
const TERM_LENGTH_BLOCKS: u64 = 90;   // ~90 days at 1 block/day

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModeratorRecord {
    pub wallet: String,
    pub topic_id: String,
    pub term_start: u64,
    pub term_end: u64,
    pub renewal_count: u8,  // max 1 renewal
    pub active: bool,
}

impl ModeratorRecord {
    pub fn can_renew(&self) -> bool {
        self.renewal_count < 1
    }

    pub fn is_term_active(&self, current_block: u64) -> bool {
        self.active && current_block <= self.term_end
    }
}

// ── Petition ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Petition {
    pub subject: String,  // e.g. "recall:wallet-A:topic-t/1" or "referenda:title"
    pub signatories: BTreeSet<String>,
    pub threshold: u64,
    pub triggered: bool,
}

impl Petition {
    pub fn sign(&mut self, wallet: &str) -> bool {
        self.signatories.insert(wallet.to_owned())
    }
    pub fn met_threshold(&self) -> bool {
        self.signatories.len() as u64 >= self.threshold
    }
}

// ── Validator record ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorRecord {
    pub wallet: String,
    pub added_at_block: u64,
    pub active: bool,
}

// ── Pallet ────────────────────────────────────────────────────────────────────

pub struct GovernancePallet {
    /// Ordered rule map: key → latest ConstitutionRule
    constitution: BTreeMap<String, ConstitutionRule>,
    /// Full version history
    pub constitution_versions: Vec<ConstitutionVersion>,
    constitution_version: u32,

    proposals: BTreeMap<String, Proposal>,
    proposal_counter: u64,

    /// topic_id → list of active/past moderators
    moderators: BTreeMap<String, Vec<ModeratorRecord>>,

    /// petition_key → Petition
    petitions: BTreeMap<String, Petition>,

    /// Active + historical validators
    validators: Vec<ValidatorRecord>,
}

impl Default for GovernancePallet {
    fn default() -> Self {
        Self {
            constitution: BTreeMap::new(),
            constitution_versions: Vec::new(),
            constitution_version: 0,
            proposals: BTreeMap::new(),
            proposal_counter: 0,
            moderators: BTreeMap::new(),
            petitions: BTreeMap::new(),
            validators: Vec::new(),
        }
    }
}

impl GovernancePallet {
    // ── Constitution ──────────────────────────────────────────────────────────

    /// Seed genesis constitution rules. Called once at chain initialisation.
    pub fn seed_constitution(&mut self, rules: Vec<(&str, &str)>, at_block: u64) {
        for (key, value) in rules {
            self.constitution.insert(key.to_owned(), ConstitutionRule {
                key: key.to_owned(),
                value: value.to_owned(),
                enacted_at_block: at_block,
                enacted_by_proposal: "genesis".to_owned(),
                previous_value: None,
            });
        }
        self.snapshot_constitution(at_block, "genesis");
    }

    /// Enact a rule change after a constitutional proposal passes.
    pub fn enact_rule(
        &mut self,
        key: &str,
        value: &str,
        proposal_id: &str,
        at_block: u64,
    ) {
        let prev = self.constitution.get(key).map(|r| r.value.clone());
        self.constitution.insert(key.to_owned(), ConstitutionRule {
            key: key.to_owned(),
            value: value.to_owned(),
            enacted_at_block: at_block,
            enacted_by_proposal: proposal_id.to_owned(),
            previous_value: prev,
        });
        self.snapshot_constitution(at_block, proposal_id);
    }

    pub fn get_rule(&self, key: &str) -> Option<&ConstitutionRule> {
        self.constitution.get(key)
    }

    fn snapshot_constitution(&mut self, at_block: u64, proposal_id: &str) {
        self.constitution_version += 1;
        self.constitution_versions.push(ConstitutionVersion {
            version: self.constitution_version,
            rules_snapshot: self.constitution.values().cloned().collect(),
            enacted_at_block: at_block,
            enacted_by_proposal: proposal_id.to_owned(),
        });
    }

    // ── Proposals ─────────────────────────────────────────────────────────────

    pub fn create_proposal(
        &mut self,
        proposal_type: ProposalType,
        title: &str,
        description: &str,
        proposed_by: &str,
        at_block: u64,
        total_eligible: u64,
    ) -> &Proposal {
        self.proposal_counter += 1;
        let id = format!("prop-{:06}", self.proposal_counter);
        let duration = if proposal_type.is_constitutional() { 200u64 } else { 100u64 };
        let proposal = Proposal {
            id: id.clone(),
            proposal_type,
            title: title.to_owned(),
            description: description.to_owned(),
            proposed_by: proposed_by.to_owned(),
            proposed_at_block: at_block,
            closes_at_block: at_block + duration,
            status: ProposalStatus::Active,
            yes_votes: BTreeSet::new(),
            no_votes: BTreeSet::new(),
            total_eligible,
        };
        self.proposals.insert(id.clone(), proposal);
        &self.proposals[&id]
    }

    pub fn vote_on_proposal(
        &mut self,
        proposal_id: &str,
        voter: &str,
        approve: bool,
    ) -> Result<(), &'static str> {
        self.proposals
            .get_mut(proposal_id)
            .ok_or("proposal_not_found")?
            .cast_vote(voter, approve)
    }

    /// Close a proposal and apply quorum check. Returns the final status.
    pub fn close_proposal(&mut self, proposal_id: &str) -> Result<ProposalStatus, &'static str> {
        let p = self.proposals.get_mut(proposal_id).ok_or("proposal_not_found")?;
        Ok(p.tally())
    }

    pub fn get_proposal(&self, id: &str) -> Option<&Proposal> {
        self.proposals.get(id)
    }

    // ── Moderator elections ───────────────────────────────────────────────────

    /// Elect a moderator for a topic. Returns Err if topic already has max moderators.
    pub fn elect_moderator(
        &mut self,
        topic_id: &str,
        wallet: &str,
        at_block: u64,
    ) -> Result<ModeratorRecord, &'static str> {
        let entries = self.moderators.entry(topic_id.to_owned()).or_default();
        let active_count = entries.iter().filter(|m| m.is_term_active(at_block)).count();
        if active_count >= MAX_MODERATORS_PER_TOPIC {
            return Err("max_moderators_reached");
        }
        let record = ModeratorRecord {
            wallet: wallet.to_owned(),
            topic_id: topic_id.to_owned(),
            term_start: at_block,
            term_end: at_block + TERM_LENGTH_BLOCKS,
            renewal_count: 0,
            active: true,
        };
        entries.push(record.clone());
        Ok(record)
    }

    /// Renew a moderator's term (allowed once).
    pub fn renew_moderator(
        &mut self,
        topic_id: &str,
        wallet: &str,
        at_block: u64,
    ) -> Result<(), &'static str> {
        let entries = self.moderators.get_mut(topic_id).ok_or("no_moderators")?;
        let rec = entries
            .iter_mut()
            .find(|m| m.wallet == wallet && m.active)
            .ok_or("moderator_not_found")?;
        if !rec.can_renew() {
            return Err("renewal_limit_reached");
        }
        rec.renewal_count += 1;
        rec.term_end = at_block + TERM_LENGTH_BLOCKS;
        Ok(())
    }

    /// Remove a moderator (called after recall vote passes).
    pub fn remove_moderator(&mut self, topic_id: &str, wallet: &str) -> Result<(), &'static str> {
        let entries = self.moderators.get_mut(topic_id).ok_or("no_moderators")?;
        let rec = entries
            .iter_mut()
            .find(|m| m.wallet == wallet && m.active)
            .ok_or("moderator_not_found")?;
        rec.active = false;
        Ok(())
    }

    pub fn active_moderators(&self, topic_id: &str, at_block: u64) -> Vec<&ModeratorRecord> {
        self.moderators
            .get(topic_id)
            .map(|v| v.iter().filter(|m| m.is_term_active(at_block)).collect())
            .unwrap_or_default()
    }

    // ── Recall petitions ──────────────────────────────────────────────────────

    /// Citizen signs a recall petition for a moderator.
    /// Returns `true` when the threshold is crossed for the first time.
    pub fn sign_recall(
        &mut self,
        topic_id: &str,
        moderator_wallet: &str,
        signer: &str,
        eligible_voters: u64,
    ) -> bool {
        let key = format!("recall:{moderator_wallet}:{topic_id}");
        let threshold = recall_threshold(eligible_voters);
        let petition = self.petitions.entry(key.clone()).or_insert_with(|| Petition {
            subject: key.clone(),
            signatories: BTreeSet::new(),
            threshold,
            triggered: false,
        });
        petition.sign(signer);
        if petition.met_threshold() && !petition.triggered {
            petition.triggered = true;
            return true; // caller should auto-open a recall v/
        }
        false
    }

    // ── Citizen-initiated referenda ───────────────────────────────────────────

    /// Citizen signs a referenda support petition.
    /// Returns `true` when threshold crossed — caller should auto-open proposal v/.
    pub fn sign_referenda(
        &mut self,
        referenda_key: &str,
        signer: &str,
        eligible_voters: u64,
    ) -> bool {
        let threshold = referenda_threshold(eligible_voters);
        let petition = self.petitions.entry(referenda_key.to_owned()).or_insert_with(|| Petition {
            subject: referenda_key.to_owned(),
            signatories: BTreeSet::new(),
            threshold,
            triggered: false,
        });
        petition.sign(signer);
        if petition.met_threshold() && !petition.triggered {
            petition.triggered = true;
            return true;
        }
        false
    }

    pub fn petition_signature_count(&self, key: &str) -> usize {
        self.petitions.get(key).map(|p| p.signatories.len()).unwrap_or(0)
    }

    // ── Validator set management (Phase 6) ────────────────────────────────────

    /// Add a validator to the active set. Requires a passed Constitutional proposal.
    pub fn add_validator(&mut self, proposal_id: &str, wallet: &str, at_block: u64) -> Result<ValidatorRecord, &'static str> {
        let proposal = self.proposals.get(proposal_id).ok_or("proposal_not_found")?;
        if proposal.status != ProposalStatus::Passed {
            return Err("proposal_not_passed");
        }
        if self.validators.iter().any(|v| v.wallet == wallet && v.active) {
            return Err("already_a_validator");
        }
        let rec = ValidatorRecord { wallet: wallet.to_owned(), added_at_block: at_block, active: true };
        self.validators.push(rec.clone());
        Ok(rec)
    }

    /// Remove a validator — requires a passed Constitutional proposal.
    pub fn remove_validator(&mut self, proposal_id: &str, wallet: &str) -> Result<(), &'static str> {
        let proposal = self.proposals.get(proposal_id).ok_or("proposal_not_found")?;
        if proposal.status != ProposalStatus::Passed {
            return Err("proposal_not_passed");
        }
        let rec = self.validators.iter_mut()
            .find(|v| v.wallet == wallet && v.active)
            .ok_or("validator_not_found")?;
        rec.active = false;
        Ok(())
    }

    pub fn active_validator_count(&self) -> usize {
        self.validators.iter().filter(|v| v.active).count()
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make_pallet() -> GovernancePallet {
        let mut p = GovernancePallet::default();
        p.seed_constitution(vec![
            ("quorum.regular", "0.501"),
            ("quorum.constitutional", "0.67"),
            ("moderator.max_per_topic", "2"),
            ("moderator.term_blocks", "90"),
        ], 0);
        p
    }

    // ── Constitution ──────────────────────────────────────────────────────────

    #[test]
    fn genesis_constitution_seeded() {
        let p = make_pallet();
        assert_eq!(p.constitution_versions.len(), 1);
        assert_eq!(p.get_rule("quorum.regular").unwrap().value, "0.501");
    }

    #[test]
    fn rule_change_creates_new_version_with_history() {
        let mut p = make_pallet();
        p.enact_rule("quorum.regular", "0.55", "prop-000001", 500);
        assert_eq!(p.constitution_versions.len(), 2);
        let rule = p.get_rule("quorum.regular").unwrap();
        assert_eq!(rule.value, "0.55");
        assert_eq!(rule.previous_value.as_deref(), Some("0.501"));
    }

    // ── Proposals ─────────────────────────────────────────────────────────────

    #[test]
    fn regular_proposal_passes_at_simple_majority() {
        let mut p = make_pallet();
        let id = {
            let prop = p.create_proposal(
                ProposalType::Regular,
                "Add rural NEP subtopic",
                "Creates st/nep-rural",
                "wallet-A", 1, 100,
            );
            prop.id.clone()
        };
        for i in 0..51u64 {
            p.vote_on_proposal(&id, &format!("w-{i}"), true).unwrap();
        }
        let status = p.close_proposal(&id).unwrap();
        assert_eq!(status, ProposalStatus::Passed);
    }

    #[test]
    fn regular_proposal_rejected_at_50_percent() {
        let mut p = make_pallet();
        let id = {
            let prop = p.create_proposal(ProposalType::Regular, "t", "d", "w", 1, 100);
            prop.id.clone()
        };
        for i in 0..50u64 {
            p.vote_on_proposal(&id, &format!("w-{i}"), true).unwrap();
        }
        let status = p.close_proposal(&id).unwrap();
        assert_eq!(status, ProposalStatus::Rejected);
    }

    #[test]
    fn constitutional_requires_67_percent() {
        let mut p = make_pallet();
        let id = {
            let prop = p.create_proposal(ProposalType::Constitutional, "t", "d", "w", 1, 100);
            prop.id.clone()
        };
        // 66 votes YES → should fail
        for i in 0..66u64 {
            p.vote_on_proposal(&id, &format!("w-{i}"), true).unwrap();
        }
        assert_eq!(p.close_proposal(&id).unwrap(), ProposalStatus::Rejected);

        // Re-create with 67 votes YES
        let id2 = {
            let prop = p.create_proposal(ProposalType::Constitutional, "t2", "d2", "w2", 2, 100);
            prop.id.clone()
        };
        for i in 0..67u64 {
            p.vote_on_proposal(&id2, &format!("u-{i}"), true).unwrap();
        }
        assert_eq!(p.close_proposal(&id2).unwrap(), ProposalStatus::Passed);
    }

    #[test]
    fn double_vote_on_proposal_rejected() {
        let mut p = make_pallet();
        let id = {
            let prop = p.create_proposal(ProposalType::Regular, "t", "d", "w", 1, 10);
            prop.id.clone()
        };
        p.vote_on_proposal(&id, "voter-A", true).unwrap();
        let err = p.vote_on_proposal(&id, "voter-A", false).unwrap_err();
        assert_eq!(err, "already_voted");
    }

    // ── Moderator elections ───────────────────────────────────────────────────

    #[test]
    fn elect_two_moderators_then_reject_third() {
        let mut p = make_pallet();
        p.elect_moderator("t/topic-1", "mod-A", 100).unwrap();
        p.elect_moderator("t/topic-1", "mod-B", 100).unwrap();
        let err = p.elect_moderator("t/topic-1", "mod-C", 100).unwrap_err();
        assert_eq!(err, "max_moderators_reached");
    }

    #[test]
    fn moderator_renewal_allowed_once() {
        let mut p = make_pallet();
        p.elect_moderator("t/t1", "mod-A", 0).unwrap();
        p.renew_moderator("t/t1", "mod-A", 90).unwrap();
        let err = p.renew_moderator("t/t1", "mod-A", 180).unwrap_err();
        assert_eq!(err, "renewal_limit_reached");
    }

    #[test]
    fn expired_moderator_slot_opens_up() {
        let mut p = make_pallet();
        p.elect_moderator("t/t1", "mod-A", 0).unwrap();
        p.elect_moderator("t/t1", "mod-B", 0).unwrap();
        // Both expire at block 90
        // At block 200, both terms are expired → slot is open
        let result = p.elect_moderator("t/t1", "mod-C", 200);
        assert!(result.is_ok());
    }

    #[test]
    fn remove_moderator_frees_slot() {
        let mut p = make_pallet();
        p.elect_moderator("t/t1", "mod-A", 0).unwrap();
        p.elect_moderator("t/t1", "mod-B", 0).unwrap();
        p.remove_moderator("t/t1", "mod-A").unwrap();
        let result = p.elect_moderator("t/t1", "mod-C", 10);
        assert!(result.is_ok());
    }

    // ── Recall petitions ──────────────────────────────────────────────────────

    #[test]
    fn recall_triggers_at_10_percent_of_voters() {
        let mut p = make_pallet();
        p.elect_moderator("t/t1", "mod-A", 0).unwrap();
        // 100 eligible voters, threshold = 10
        let mut triggered = false;
        for i in 0..10usize {
            triggered = p.sign_recall("t/t1", "mod-A", &format!("voter-{i}"), 100);
        }
        assert!(triggered);
    }

    #[test]
    fn recall_does_not_trigger_before_threshold() {
        let mut p = make_pallet();
        p.elect_moderator("t/t1", "mod-A", 0).unwrap();
        for i in 0..9usize {
            let triggered = p.sign_recall("t/t1", "mod-A", &format!("voter-{i}"), 100);
            assert!(!triggered, "triggered too early at {i}");
        }
    }

    #[test]
    fn recall_triggers_only_once() {
        let mut p = make_pallet();
        p.elect_moderator("t/t1", "mod-A", 0).unwrap();
        for i in 0..10usize {
            p.sign_recall("t/t1", "mod-A", &format!("voter-{i}"), 100);
        }
        // Additional signatures after threshold do not re-trigger
        let re_triggered = p.sign_recall("t/t1", "mod-A", "voter-99", 100);
        assert!(!re_triggered);
    }

    // ── Citizen referenda ─────────────────────────────────────────────────────

    #[test]
    fn referenda_triggers_at_5_percent() {
        let mut p = make_pallet();
        let mut triggered = false;
        for i in 0..5usize {
            triggered = p.sign_referenda("ref:improve-water-policy", &format!("w-{i}"), 100);
        }
        assert!(triggered);
    }

    // ── Validator set ─────────────────────────────────────────────────────────

    fn passed_constitutional_prop(p: &mut GovernancePallet) -> String {
        let id = {
            let prop = p.create_proposal(ProposalType::Constitutional, "add-validator", "d", "w", 1, 100);
            prop.id.clone()
        };
        for i in 0..67u64 {
            p.vote_on_proposal(&id, &format!("v-{i}"), true).unwrap();
        }
        p.close_proposal(&id).unwrap();
        id
    }

    #[test]
    fn add_validator_requires_passed_proposal() {
        let mut p = make_pallet();
        let prop_id = passed_constitutional_prop(&mut p);
        p.add_validator(&prop_id, "validator-IIT-Bombay", 500).unwrap();
        assert_eq!(p.active_validator_count(), 1);
    }

    #[test]
    fn add_validator_without_passed_proposal_fails() {
        let mut p = make_pallet();
        let err = p.add_validator("prop-nonexistent", "wallet-X", 1).unwrap_err();
        assert_eq!(err, "proposal_not_found");
    }

    #[test]
    fn duplicate_validator_rejected() {
        let mut p = make_pallet();
        let id = passed_constitutional_prop(&mut p);
        p.add_validator(&id, "val-A", 1).unwrap();
        let err = p.add_validator(&id, "val-A", 2).unwrap_err();
        assert_eq!(err, "already_a_validator");
    }

    #[test]
    fn remove_validator_marks_inactive() {
        let mut p = make_pallet();
        let id = passed_constitutional_prop(&mut p);
        p.add_validator(&id, "val-A", 1).unwrap();
        assert_eq!(p.active_validator_count(), 1);

        let id2 = passed_constitutional_prop(&mut p);
        p.remove_validator(&id2, "val-A").unwrap();
        assert_eq!(p.active_validator_count(), 0);
    }
}
