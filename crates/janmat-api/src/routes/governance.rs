#![allow(dead_code)]
use axum::{Json, extract::Path, http::StatusCode};
use serde::{Deserialize, Serialize};

// ── Response types ────────────────────────────────────────────────────────────

#[derive(Serialize)]
pub struct ConstitutionResponse {
    pub version: u32,
    pub rules: Vec<RuleEntry>,
}

#[derive(Serialize)]
pub struct RuleEntry {
    pub key: String,
    pub value: String,
    pub enacted_at_block: u64,
    pub enacted_by_proposal: String,
}

#[derive(Deserialize)]
pub struct CreateProposalRequest {
    pub proposal_type: String,  // "regular" | "constitutional"
    pub title: String,
    pub description: String,
    pub proposer_wallet: String,
}

#[derive(Serialize)]
pub struct CreateProposalResponse {
    pub proposal_id: String,
    pub closes_at_block: u64,
    pub required_quorum: f64,
}

#[derive(Deserialize)]
pub struct RecallRequest {
    pub moderator_wallet: String,
    pub signer_wallet: String,
}

#[derive(Serialize)]
pub struct RecallResponse {
    pub signatures: usize,
    pub threshold: u64,
    pub recall_vote_opened: bool,
}

// ── Handlers ──────────────────────────────────────────────────────────────────

/// GET /pub/constitution — current constitution rules
pub async fn get_constitution() -> Json<ConstitutionResponse> {
    use janmat_governance::{REGULAR_QUORUM, CONSTITUTIONAL_QUORUM};

    // Phase 5: return genesis seed rules; Phase 6 reads from chain indexer
    let rules = vec![
        RuleEntry { key: "quorum.regular".into(),           value: format!("{REGULAR_QUORUM}"),        enacted_at_block: 0, enacted_by_proposal: "genesis".into() },
        RuleEntry { key: "quorum.constitutional".into(),    value: format!("{CONSTITUTIONAL_QUORUM}"),  enacted_at_block: 0, enacted_by_proposal: "genesis".into() },
        RuleEntry { key: "moderator.max_per_topic".into(),  value: "2".into(),                         enacted_at_block: 0, enacted_by_proposal: "genesis".into() },
        RuleEntry { key: "moderator.term_blocks".into(),    value: "90".into(),                        enacted_at_block: 0, enacted_by_proposal: "genesis".into() },
        RuleEntry { key: "recall.petition_ratio".into(),    value: "0.10".into(),                      enacted_at_block: 0, enacted_by_proposal: "genesis".into() },
        RuleEntry { key: "referenda.support_ratio".into(),  value: "0.05".into(),                      enacted_at_block: 0, enacted_by_proposal: "genesis".into() },
    ];
    Json(ConstitutionResponse { version: 1, rules })
}

/// POST /citizen/proposals — create a new governance proposal
pub async fn create_proposal(
    Json(req): Json<CreateProposalRequest>,
) -> Result<Json<CreateProposalResponse>, StatusCode> {
    use janmat_governance::{ProposalType, REGULAR_QUORUM, CONSTITUTIONAL_QUORUM};

    let (proposal_type, quorum) = match req.proposal_type.as_str() {
        "constitutional" => (ProposalType::Constitutional, CONSTITUTIONAL_QUORUM),
        "regular"        => (ProposalType::Regular, REGULAR_QUORUM),
        _                => return Err(StatusCode::BAD_REQUEST),
    };

    let duration = if proposal_type.is_constitutional() { 200u64 } else { 100u64 };
    let proposal_id = format!("prop-api-{}", ulid::Ulid::new());

    Ok(Json(CreateProposalResponse {
        proposal_id,
        closes_at_block: duration,
        required_quorum: quorum,
    }))
}

/// GET /pub/proposals — list active proposals (stub; Phase 6 reads from indexer)
pub async fn list_proposals() -> Json<Vec<serde_json::Value>> {
    Json(vec![])
}

/// POST /citizen/recall/:topic_id — sign a recall petition
pub async fn sign_recall(
    Path(_topic_id): Path<String>,
    Json(_req): Json<RecallRequest>,
) -> Json<RecallResponse> {
    use janmat_governance::recall_threshold;
    // Phase 5: in-memory only; Phase 6 persists via pallet-governance extrinsic
    let threshold = recall_threshold(1000); // assume 1000 eligible for now
    Json(RecallResponse {
        signatures: 1,
        threshold,
        recall_vote_opened: false,
    })
}
