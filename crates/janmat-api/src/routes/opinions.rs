#![allow(dead_code)]
use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SubmitOpinionRequest {
    pub subtopic_id: String,
    pub content: String,
    pub creator_nullifier: String,
    pub clone_acknowledged: bool,
}

#[derive(Serialize)]
pub struct SubmitOpinionResponse {
    pub opinion_id: String,
    pub content_cid: String,
    pub content_hash: String,
    pub verdict: String,
    pub clone_candidates: Vec<String>,
}

/// POST /citizen/o — submit an opinion through the content pipeline
/// Phase 3: content pipeline runs in-process; IPFS + chain calls are mocked.
/// Phase 4 will wire the real pallet-namespace + IPFS client.
pub async fn submit_opinion(
    Json(req): Json<SubmitOpinionRequest>,
) -> Result<Json<SubmitOpinionResponse>, StatusCode> {
    use janmat_content::{
        ContentPipeline, SubmitRequest, Verdict,
        blocklist::BlocklistEngine,
    };
    use janmat_storage::{MockIpfsClient, IpfsClient};
    use janmat_types::namespace::OpinionId;

    let pipeline = ContentPipeline::new(BlocklistEngine::default());
    let check = pipeline.check(&SubmitRequest {
        subtopic_id: &req.subtopic_id,
        text: &req.content,
    });

    match check.verdict {
        Verdict::Blocked => return Err(StatusCode::UNPROCESSABLE_ENTITY),
        Verdict::NeedsReview => return Err(StatusCode::ACCEPTED), // 202: held for review
        Verdict::CloneWarning if !req.clone_acknowledged => {
            // Return 409 with clone candidates so UI can prompt acknowledgement
            return Err(StatusCode::CONFLICT);
        }
        _ => {}
    }

    // Store content on IPFS (mock in Phase 3)
    let ipfs = MockIpfsClient::default();
    let obj = ipfs.put(req.content.as_bytes());

    let opinion_id = OpinionId::new();

    Ok(Json(SubmitOpinionResponse {
        opinion_id: opinion_id.to_string(),
        content_cid: obj.cid,
        content_hash: obj.content_hash,
        verdict: format!("{:?}", check.verdict),
        clone_candidates: check.clone_candidates,
    }))
}
