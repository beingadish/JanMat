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
}

#[derive(Deserialize)]
pub struct CastVoteRequest {
    pub nullifier: String,
    pub commitment: String,
}

#[derive(Serialize)]
pub struct CastVoteResponse {
    pub accepted: bool,
    pub block: u64,
}

// ── Handlers ──────────────────────────────────────────────────────────────────

/// POST /citizen/v — create a vote on an opinion
pub async fn create_vote(
    Json(req): Json<CreateVoteRequest>,
) -> Result<Json<CreateVoteResponse>, StatusCode> {
    use janmat_types::namespace::VoteId;
    let id = VoteId::new();
    Ok(Json(CreateVoteResponse {
        vote_id: id.to_string(),
    }))
}

/// POST /citizen/v/:id/cast — cast a vote
pub async fn cast_vote(
    Path(vote_id): Path<String>,
    Json(req): Json<CastVoteRequest>,
) -> Result<Json<CastVoteResponse>, StatusCode> {
    // Phase 1: always accept; Phase 2 will wire to pallet-voting + ZK proof
    Ok(Json(CastVoteResponse {
        accepted: true,
        block: 0,
    }))
}

/// POST /citizen/v/:id/reveal — one-way visibility toggle (creator only)
pub async fn reveal_vote(Path(vote_id): Path<String>) -> StatusCode {
    // Phase 4 will enforce creator auth and pallet-voting reveal_toggle
    StatusCode::ACCEPTED
}

/// GET /pub/t/:topic_id/v/:vote_id/results — public tally (post-close only)
pub async fn vote_results(
    Path((topic_id, vote_id)): Path<(String, String)>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "vote_id": vote_id,
        "status": "open",
        "tally": null,
    }))
}
