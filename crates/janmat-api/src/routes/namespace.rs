#![allow(dead_code)]
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
    Json(_req): Json<CreateTopicRequest>,
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

/// GET /pub/t — list topics and circles
pub async fn list_topics() -> Json<serde_json::Value> {
    Json(serde_json::json!([
        {
            "id": "infrastructure",
            "slug": "infrastructure",
            "name": "Civic Infrastructure",
            "description": "Roads, power, civic grid, and public transit modernization",
            "count": 12,
            "icon": "domain",
            "subtopics": [{ "id": "roads", "slug": "roads" }, { "id": "water", "slug": "water" }]
        },
        {
            "id": "transit",
            "slug": "transit",
            "name": "Transit Council",
            "description": "Metro connectivity, electric buses, and suburban safety",
            "count": 8,
            "icon": "directions_subway",
            "subtopics": [{ "id": "metro", "slug": "metro" }, { "id": "buses", "slug": "buses" }]
        },
        {
            "id": "it-ethics",
            "slug": "it-ethics",
            "name": "Municipal IT & AI Ethics",
            "description": "Algorithmic sovereignty, open-source audits, and automated systems",
            "count": 15,
            "icon": "smart_toy",
            "subtopics": [{ "id": "ai-audit", "slug": "ai-audit" }, { "id": "privacy", "slug": "privacy" }]
        },
        {
            "id": "energy",
            "slug": "energy",
            "name": "Clean Energy & Water",
            "description": "Neighborhood solar microgrids, water quality, and clean air initiatives",
            "count": 6,
            "icon": "energy_savings_leaf",
            "subtopics": [{ "id": "solar", "slug": "solar" }, { "id": "recycling", "slug": "recycling" }]
        },
        {
            "id": "healthcare",
            "slug": "healthcare",
            "name": "Healthcare Mandates",
            "description": "Primary healthcare centers, emergency transit, and public sanitation",
            "count": 9,
            "icon": "health_and_safety",
            "subtopics": [{ "id": "clinics", "slug": "clinics" }, { "id": "sanitation", "slug": "sanitation" }]
        }
    ]))
}

/// GET /pub/t/:id — get topic by id (stub)
pub async fn get_topic(Path(id): Path<String>) -> Json<serde_json::Value> {
    Json(serde_json::json!({ "topic_id": id }))
}
