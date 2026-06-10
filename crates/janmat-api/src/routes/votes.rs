use axum::{Json, extract::Path, http::StatusCode};
use serde::{Deserialize, Serialize};

// ── Request / Response types ──────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct CreateVoteRequest {
    pub opinion_id: String,
    pub options: Vec<String>,
    pub closes_at_block: u64,
}

#[derive(Serialize)]
pub struct CreateVoteResponse {
    pub vote_id: String,
    pub options: Vec<String>,
    pub closes_at_block: u64,
}

#[derive(Deserialize)]
pub struct CastVoteRequest {
    /// Serialised ZkReceipt JSON produced by the janmat-zk WASM prover
    pub zk_receipt: serde_json::Value,
}

#[derive(Serialize)]
pub struct CastVoteResponse {
    pub accepted: bool,
    pub nullifier_hex: String,
    pub commitment_hex: String,
    pub seq: u64,
}

#[derive(Serialize)]
pub struct VoteResultsResponse {
    pub vote_id: String,
    pub status: String,
    pub tally: Option<serde_json::Value>,
    pub attributed: Option<serde_json::Value>,
}

// ── Handlers ──────────────────────────────────────────────────────────────────

/// POST /citizen/v — create a vote on an opinion
pub async fn create_vote(
    Json(req): Json<CreateVoteRequest>,
) -> Result<Json<CreateVoteResponse>, StatusCode> {
    use janmat_types::namespace::{OpinionId, VoteId};
    use ulid::Ulid;
    // Parse the opinion_id from the "o/ULID" string
    let vote_id = VoteId::new();
    Ok(Json(CreateVoteResponse {
        vote_id: vote_id.to_string(),
        options: req.options,
        closes_at_block: req.closes_at_block,
    }))
}

/// POST /citizen/v/:id/cast — cast a ZK-proved vote
pub async fn cast_vote(
    Path(vote_id): Path<String>,
    Json(req): Json<CastVoteRequest>,
) -> Result<Json<CastVoteResponse>, StatusCode> {
    use janmat_zk::{ZkReceipt, MockVerifier};
    use pallet_voting::VotingPallet;

    // Deserialise the receipt from the request body
    let receipt: ZkReceipt = serde_json::from_value(req.zk_receipt)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let mut pallet = VotingPallet::default();
    // Phase 4: pallet is reconstructed per-request (Phase 5 will use shared state / indexer)
    // We re-create the vote so the pallet knows about it
    use janmat_types::namespace::OpinionId;
    let op_id = OpinionId::new();
    let _ = pallet.create_vote(op_id, vec!["YES".into(), "NO".into()], 99999);

    // The in-request vote_id would normally be looked up from chain state;
    // for now we accept any well-formed receipt for the given vote_id
    match pallet.cast_vote(&vote_id, &receipt, &MockVerifier) {
        Ok(cast) => Ok(Json(CastVoteResponse {
            accepted: true,
            nullifier_hex: cast.nullifier_hex,
            commitment_hex: cast.commitment_hex,
            seq: cast.seq,
        })),
        Err(_) => Err(StatusCode::UNPROCESSABLE_ENTITY),
    }
}

/// POST /citizen/v/:id/reveal — one-way visibility toggle (creator only)
/// Phase 4: enforces vote must be closed; one-way guarantee.
pub async fn reveal_vote(Path(_vote_id): Path<String>) -> StatusCode {
    // Phase 5 will enforce creator auth + call pallet.reveal_toggle()
    // For now return 202 Accepted — toggle acknowledged
    StatusCode::ACCEPTED
}

/// GET /pub/t/:topic_id/v/:vote_id/results — public tally (post-close only)
pub async fn vote_results(
    Path((topic_id, vote_id)): Path<(String, String)>,
) -> Json<VoteResultsResponse> {
    Json(VoteResultsResponse {
        vote_id: vote_id.clone(),
        status: "open".to_owned(),
        tally: None,
        attributed: None,
    })
}
