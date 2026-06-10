use axum::{Json, extract::Path, http::StatusCode};
use serde::{Deserialize, Serialize};

// ── Request / Response types ──────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct CreateTopicRequest {
    pub slug: String,
}

#[derive(Serialize)]
pub struct CreateTopicResponse {
    pub topic_id: String,
    pub anchor_hex: String,
}

#[derive(Deserialize)]
pub struct SubmitOpinionRequest {
    pub subtopic_id: String,
    pub content: String,
    pub creator_nullifier: String,
}

#[derive(Serialize)]
pub struct SubmitOpinionResponse {
    pub opinion_id: String,
    pub content_cid: String,
    pub content_hash: String,
}

// ── Handlers ──────────────────────────────────────────────────────────────────

/// POST /citizen/t — create a new topic
/// Phase 1: no ZK auth yet; nullifier + wallet are mocked.
pub async fn create_topic(
    Json(req): Json<CreateTopicRequest>,
) -> Result<Json<CreateTopicResponse>, StatusCode> {
    use janmat_types::AnchoredUlid;
    use janmat_types::namespace::TopicId;

    // In Phase 1 block_hash is a fixed seed; Phase 2 will fetch from chain.
    let anchored = AnchoredUlid::new(b"genesis-block");
    let id = TopicId(anchored.id);
    Ok(Json(CreateTopicResponse {
        topic_id: id.to_string(),
        anchor_hex: anchored.anchor_hex.clone(),
    }))
}

/// GET /pub/t — list topics (stub; Phase 1 indexer will back this)
pub async fn list_topics() -> Json<Vec<String>> {
    Json(vec![])
}

/// GET /pub/t/:id — get topic by id (stub)
pub async fn get_topic(Path(id): Path<String>) -> Json<serde_json::Value> {
    Json(serde_json::json!({ "topic_id": id }))
}
