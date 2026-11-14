//! Civic Pulse & Democratic Deliberation routes
//!
//! Provides real-time precinct telemetry, Blake3 cryptographic integrity proofs,
//! 3-way consensus tallies (Agree / Oppose / Neutral), and citizen motion drafting.

use std::sync::{Arc, LazyLock, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

// ── Models ───────────────────────────────────────────────────────────────────

#[derive(Clone, Serialize, Deserialize)]
pub struct AuthorMeta {
    pub name: String,
    pub role: String,
    pub verified: bool,
    pub avatar_initials: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct BallotTally {
    pub agree_pct: u8,
    pub oppose_pct: u8,
    pub neutral_pct: u8,
    pub agree_votes: u64,
    pub oppose_votes: u64,
    pub neutral_votes: u64,
    pub total_ballots: u64,
    pub quorum_pct: u16,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TopDissent {
    pub category: String,
    pub upvotes: u32,
    pub quote: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct BallotItem {
    pub id: String,
    pub proposal_code: String,
    pub circle: String,
    pub circle_id: String,
    pub title: String,
    pub summary: String,
    pub author: AuthorMeta,
    pub status: String, // "active" | "deliberation" | "passed" | "archived"
    pub status_label: String,
    pub closes_in: String,
    pub category: String, // "all" | "transit" | "tech" | "civic" | "energy"
    pub tally: BallotTally,
    pub deliberation_count: u32,
    pub blake3_hash: String,
    pub top_dissent: Option<TopDissent>,
    pub created_at: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FeaturedCircle {
    pub id: String,
    pub name: String,
    pub count: u32,
    pub icon: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct VerificationReceipt {
    pub tx_id: String,
    pub action: String,
    pub block: String,
    pub delta: String,
    pub time_ago: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TopNuancedDissent {
    pub topic: String,
    pub category: String,
    pub upvotes: u32,
    pub quote: String,
    pub thread_link: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PulseResponse {
    pub block_height: u64,
    pub precinct_id: String,
    pub precinct_name: String,
    pub jurisdiction: String,
    pub status: String,
    pub network_label: String,
    pub turnout_pct: f32,
    pub consensus_index: u8,
    pub weekly_change: String,
    pub velocity: String,
    pub zk_verifier_health: String,
    pub blake3_anchor_hash: String,
    pub featured_circles: Vec<FeaturedCircle>,
    pub recent_receipts: Vec<VerificationReceipt>,
    pub top_dissent: TopNuancedDissent,
    pub timestamp: String,
}

#[derive(Deserialize)]
pub struct CastBallotRequest {
    pub ballot_id: String,
    pub choice: String, // "agree" | "oppose" | "neutral"
    pub voter_token: Option<String>,
}

#[derive(Serialize)]
pub struct CastBallotResponse {
    pub accepted: bool,
    pub ballot_id: String,
    pub choice: String,
    pub receipt_hash: String,
    pub block_height: u64,
    pub timestamp: String,
    pub updated_tally: BallotTally,
}

#[derive(Deserialize)]
pub struct DraftMotionRequest {
    pub title: String,
    pub summary: String,
    pub circle_id: String,
    pub author_name: Option<String>,
}

#[derive(Serialize)]
pub struct DraftMotionResponse {
    pub accepted: bool,
    pub ballot: BallotItem,
}

#[derive(Deserialize)]
pub struct BallotFilterQuery {
    pub category: Option<String>,
    pub status: Option<String>,
}

// ── In-Memory Consensus Engine State ─────────────────────────────────────────

pub struct PulseEngine {
    pub genesis_block: u64,
    pub start_time: u64,
    pub ballots: Vec<BallotItem>,
    pub receipts: Vec<VerificationReceipt>,
    pub vote_seq: u64,
}

fn compute_blake3_audit(content: &str) -> String {
    let hash = blake3::hash(content.as_bytes());
    hash.to_hex().to_string()
}

fn recalculate_tally(t: &mut BallotTally) {
    let total = t.agree_votes + t.oppose_votes + t.neutral_votes;
    t.total_ballots = total;
    if total > 0 {
        t.agree_pct = ((t.agree_votes as f64 / total as f64) * 100.0).round() as u8;
        t.oppose_pct = ((t.oppose_votes as f64 / total as f64) * 100.0).round() as u8;
        // Ensure exact 100% sum
        if t.agree_pct + t.oppose_pct <= 100 {
            t.neutral_pct = 100 - (t.agree_pct + t.oppose_pct);
        } else {
            t.neutral_pct = 0;
        }
    }
}

impl PulseEngine {
    pub fn new() -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let b1_content = "TC-2024-88:Should overnight suburban metro runs be funded via dynamic weekend congestion tolls?";
        let b2_content = "ETH-2024-14:Public municipal IT must mandate open-source audits before deploying automated AI workflows.";
        let b3_content = "ENG-2024-03:Allocate 15% of annual municipal parking revenue towards neighborhood solar microgrids.";

        let ballots = vec![
            BallotItem {
                id: "tc-2024-88".to_string(),
                proposal_code: "TC-2024-88".to_string(),
                circle: "Transit Council".to_string(),
                circle_id: "transit".to_string(),
                title: "Should overnight suburban metro runs be funded via dynamic weekend congestion tolls?".to_string(),
                summary: "High demand for night shift workers and safer weekend commute alternatives justifies off-peak operation, offset by peak suburban highway congestion charges during non-commercial hours.".to_string(),
                author: AuthorMeta {
                    name: "Aarav K.".to_string(),
                    role: "Transit Council Representative".to_string(),
                    verified: true,
                    avatar_initials: "AK".to_string(),
                },
                status: "active".to_string(),
                status_label: "Active Ballot • Closes in 18h".to_string(),
                closes_in: "18h".to_string(),
                category: "transit".to_string(),
                tally: BallotTally {
                    agree_pct: 68,
                    oppose_pct: 26,
                    neutral_pct: 6,
                    agree_votes: 2320,
                    oppose_votes: 887,
                    neutral_votes: 205,
                    total_ballots: 3412,
                    quorum_pct: 114,
                },
                deliberation_count: 142,
                blake3_hash: compute_blake3_audit(b1_content),
                top_dissent: Some(TopDissent {
                    category: "Economic Sustainability".to_string(),
                    upvotes: 142,
                    quote: "Dynamic congestion tolls during economic downturns create an undue regressive penalty on non-salaried weekend service providers who cannot shift commuting hours.".to_string(),
                }),
                created_at: "2026-11-14T06:00:00Z".to_string(),
            },
            BallotItem {
                id: "eth-2024-14".to_string(),
                proposal_code: "ETH-2024-14".to_string(),
                circle: "Municipal IT & AI Ethics".to_string(),
                circle_id: "it-ethics".to_string(),
                title: "Public municipal IT must mandate open-source audits before deploying automated AI workflows.".to_string(),
                summary: "Algorithmic public benefit disbursement and traffic management systems require independent peer-reviewed transparency reports before rollout to eliminate systemic demographic bias.".to_string(),
                author: AuthorMeta {
                    name: "Meera R.".to_string(),
                    role: "Tech & Ethics Civic Fellow".to_string(),
                    verified: true,
                    avatar_initials: "MR".to_string(),
                },
                status: "deliberation".to_string(),
                status_label: "Open Deliberation".to_string(),
                closes_in: "2d".to_string(),
                category: "tech".to_string(),
                tally: BallotTally {
                    agree_pct: 84,
                    oppose_pct: 11,
                    neutral_pct: 5,
                    agree_votes: 1588,
                    oppose_votes: 208,
                    neutral_votes: 94,
                    total_ballots: 1890,
                    quorum_pct: 98,
                },
                deliberation_count: 89,
                blake3_hash: compute_blake3_audit(b2_content),
                top_dissent: Some(TopDissent {
                    category: "Execution Latency".to_string(),
                    upvotes: 48,
                    quote: "Mandatory multi-month open audit timelines could stall critical municipal cybersecurity response updates when rapid zero-day patching is required.".to_string(),
                }),
                created_at: "2026-11-14T04:00:00Z".to_string(),
            },
            BallotItem {
                id: "eng-2024-03".to_string(),
                proposal_code: "ENG-2024-03".to_string(),
                circle: "Clean Energy & Water".to_string(),
                circle_id: "energy".to_string(),
                title: "Allocate 15% of annual municipal parking revenue towards neighborhood solar microgrids.".to_string(),
                summary: "Provides dedicated seed capital for community battery storage and rooftop photovoltaic cells in historically under-electrified wards.".to_string(),
                author: AuthorMeta {
                    name: "Rohan Sen".to_string(),
                    role: "Civic Green Energy Board".to_string(),
                    verified: true,
                    avatar_initials: "RS".to_string(),
                },
                status: "passed".to_string(),
                status_label: "Quorum Met".to_string(),
                closes_in: "Passed".to_string(),
                category: "civic".to_string(),
                tally: BallotTally {
                    agree_pct: 72,
                    oppose_pct: 21,
                    neutral_pct: 7,
                    agree_votes: 2965,
                    oppose_votes: 865,
                    neutral_votes: 288,
                    total_ballots: 4118,
                    quorum_pct: 145,
                },
                deliberation_count: 204,
                blake3_hash: compute_blake3_audit(b3_content),
                top_dissent: None,
                created_at: "2026-11-13T12:00:00Z".to_string(),
            },
        ];

        let receipts = vec![
            VerificationReceipt {
                tx_id: "#4092".to_string(),
                action: "Cast ballot in Transit Council".to_string(),
                block: "0x9f4a...8d12".to_string(),
                delta: "Quorum +0.02%".to_string(),
                time_ago: "14s ago".to_string(),
            },
            VerificationReceipt {
                tx_id: "#1884".to_string(),
                action: "Submitted argument counter-point".to_string(),
                block: "0x3c2a...e901".to_string(),
                delta: "Thread: AI Open Source Mandates".to_string(),
                time_ago: "38s ago".to_string(),
            },
            VerificationReceipt {
                tx_id: "#9021".to_string(),
                action: "Precinct 42 Citizen Node Verified".to_string(),
                block: "0x7b11...fa44".to_string(),
                delta: "Anti-Sybil Proof Confirmed".to_string(),
                time_ago: "1m ago".to_string(),
            },
        ];

        Self {
            genesis_block: 849_201,
            start_time: now,
            ballots,
            receipts,
            vote_seq: 1042,
        }
    }

    pub fn current_block(&self) -> u64 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let elapsed_seconds = now.saturating_sub(self.start_time);
        // Progress 1 block every 6 seconds on the civic ledger
        self.genesis_block + (elapsed_seconds / 6)
    }
}

static ENGINE: LazyLock<Arc<RwLock<PulseEngine>>> =
    LazyLock::new(|| Arc::new(RwLock::new(PulseEngine::new())));

// ── Handlers ──────────────────────────────────────────────────────────────────

/// GET /pub/pulse — real-time precinct telemetry, active status, and cryptographic metrics
pub async fn get_pulse() -> Json<PulseResponse> {
    let engine = ENGINE.read().expect("read pulse engine");
    let block = engine.current_block();

    let featured_circles = vec![
        FeaturedCircle {
            id: "infrastructure".to_string(),
            name: "Civic Infrastructure".to_string(),
            count: 12,
            icon: "domain".to_string(),
        },
        FeaturedCircle {
            id: "transit".to_string(),
            name: "Transit Council".to_string(),
            count: 8,
            icon: "directions_subway".to_string(),
        },
        FeaturedCircle {
            id: "it-ethics".to_string(),
            name: "Municipal IT & AI Ethics".to_string(),
            count: 15,
            icon: "smart_toy".to_string(),
        },
        FeaturedCircle {
            id: "energy".to_string(),
            name: "Clean Energy & Water".to_string(),
            count: 6,
            icon: "energy_savings_leaf".to_string(),
        },
        FeaturedCircle {
            id: "healthcare".to_string(),
            name: "Healthcare Mandates".to_string(),
            count: 9,
            icon: "health_and_safety".to_string(),
        },
    ];

    let anchor_input = format!("precinct:42:block:{block}:status:active");
    let blake3_anchor_hash = compute_blake3_audit(&anchor_input);

    let top_dissent = TopNuancedDissent {
        topic: "Suburban Metro".to_string(),
        category: "Economic Sustainability".to_string(),
        upvotes: 142,
        quote: "Dynamic congestion tolls during economic downturns create an undue regressive penalty on non-salaried weekend service providers who cannot shift commuting hours.".to_string(),
        thread_link: "/pub/ballots/tc-2024-88".to_string(),
    };

    Json(PulseResponse {
        block_height: block,
        precinct_id: "Precinct 42-North".to_string(),
        precinct_name: "Precinct 42".to_string(),
        jurisdiction: "Urban Metro Central Jurisdiction".to_string(),
        status: "ACTIVE".to_string(),
        network_label: "PRECINCT 42 NETWORK ACTIVE".to_string(),
        turnout_pct: 74.2,
        consensus_index: 82,
        weekly_change: "+4.3".to_string(),
        velocity: "High Active".to_string(),
        zk_verifier_health: "operational".to_string(),
        blake3_anchor_hash,
        featured_circles,
        recent_receipts: engine.receipts.clone(),
        top_dissent,
        timestamp: chrono_now(),
    })
}

/// GET /pub/ballots — list active ballots with optional category/status filters
pub async fn list_ballots(Query(query): Query<BallotFilterQuery>) -> Json<Vec<BallotItem>> {
    let engine = ENGINE.read().expect("read pulse engine");
    let mut items = engine.ballots.clone();

    if let Some(cat) = query.category.filter(|c| !c.is_empty() && c != "all") {
        items.retain(|b| b.category.eq_ignore_ascii_case(&cat) || b.circle_id.eq_ignore_ascii_case(&cat));
    }

    if let Some(st) = query.status.filter(|s| !s.is_empty() && s != "all") {
        items.retain(|b| b.status.eq_ignore_ascii_case(&st));
    }

    Json(items)
}

/// GET /pub/ballots/:id — single ballot details
pub async fn get_ballot(Path(id): Path<String>) -> Result<Json<BallotItem>, StatusCode> {
    let engine = ENGINE.read().expect("read pulse engine");
    let found = engine
        .ballots
        .iter()
        .find(|b| b.id.eq_ignore_ascii_case(&id) || b.proposal_code.eq_ignore_ascii_case(&id))
        .cloned();

    match found {
        Some(item) => Ok(Json(item)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// POST /citizen/ballots/vote — cast a verified 3-way ballot choice with Blake3 receipt
pub async fn cast_ballot_vote(
    Json(req): Json<CastBallotRequest>,
) -> Result<Json<CastBallotResponse>, StatusCode> {
    let mut engine = ENGINE.write().expect("write pulse engine");
    let block = engine.current_block();
    engine.vote_seq += 1;
    let seq = engine.vote_seq;

    let choice = req.choice.trim().to_lowercase();
    if choice != "agree" && choice != "oppose" && choice != "neutral" {
        return Err(StatusCode::BAD_REQUEST);
    }

    let ballot_idx = engine
        .ballots
        .iter()
        .position(|b| b.id.eq_ignore_ascii_case(&req.ballot_id) || b.proposal_code.eq_ignore_ascii_case(&req.ballot_id))
        .ok_or(StatusCode::NOT_FOUND)?;

    let ballot_id = {
        let ballot = &mut engine.ballots[ballot_idx];

        match choice.as_str() {
            "agree" => ballot.tally.agree_votes += 1,
            "oppose" => ballot.tally.oppose_votes += 1,
            "neutral" => ballot.tally.neutral_votes += 1,
            _ => {}
        }

        recalculate_tally(&mut ballot.tally);
        ballot.id.clone()
    };

    let updated_tally = engine.ballots[ballot_idx].tally.clone();
    let ballot_title = engine.ballots[ballot_idx].title.clone();

    // Generate deterministic Blake3 cryptographic ballot audit receipt
    let receipt_input = format!(
        "vote:{}:{}:{}:{}:block:{}",
        ballot_id, choice, seq, req.voter_token.as_deref().unwrap_or("anon_citizen"), block
    );
    let receipt_hash = compute_blake3_audit(&receipt_input);

    // Prepend to verified receipts stream
    let short_hash = if receipt_hash.len() > 10 {
        format!("0x{}...{}", &receipt_hash[..4], &receipt_hash[receipt_hash.len() - 4..])
    } else {
        receipt_hash.clone()
    };

    engine.receipts.insert(
        0,
        VerificationReceipt {
            tx_id: format!("#{seq}"),
            action: format!("Cast ballot: {}", &ballot_title[..ballot_title.len().min(28)]),
            block: short_hash,
            delta: format!("Choice: {choice}"),
            time_ago: "Just now".to_string(),
        },
    );
    if engine.receipts.len() > 10 {
        engine.receipts.truncate(10);
    }

    Ok(Json(CastBallotResponse {
        accepted: true,
        ballot_id,
        choice,
        receipt_hash,
        block_height: block,
        timestamp: chrono_now(),
        updated_tally,
    }))
}

/// POST /citizen/ballots/draft — create a new citizen motion / ballot
pub async fn draft_ballot_motion(
    Json(req): Json<DraftMotionRequest>,
) -> Result<Json<DraftMotionResponse>, StatusCode> {
    if req.title.trim().is_empty() || req.summary.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let mut engine = ENGINE.write().expect("write pulse engine");
    let block = engine.current_block();
    engine.vote_seq += 1;
    let seq = engine.vote_seq;

    let code = format!("CIT-2026-{seq}");
    let id = format!("cit-2026-{seq}");

    let circle_name = match req.circle_id.to_lowercase().as_str() {
        "transit" => "Transit Council",
        "it-ethics" | "tech" => "Municipal IT & AI Ethics",
        "energy" => "Clean Energy & Water",
        "healthcare" => "Healthcare Mandates",
        _ => "Civic Infrastructure",
    };

    let author_name = req
        .author_name
        .filter(|n| !n.trim().is_empty())
        .unwrap_or_else(|| "Citizen Sponsor".to_string());

    let initials = author_name
        .split_whitespace()
        .filter_map(|w| w.chars().next())
        .take(2)
        .collect::<String>()
        .to_uppercase();

    let audit_hash = compute_blake3_audit(&format!("{code}:{}:{}", req.title, req.summary));

    let new_ballot = BallotItem {
        id: id.clone(),
        proposal_code: code.clone(),
        circle: circle_name.to_string(),
        circle_id: req.circle_id.clone(),
        title: req.title.clone(),
        summary: req.summary.clone(),
        author: AuthorMeta {
            name: author_name,
            role: "Citizen Sponsor".to_string(),
            verified: true,
            avatar_initials: if initials.is_empty() { "CS".into() } else { initials },
        },
        status: "active".to_string(),
        status_label: "Active Ballot • Closes in 48h".to_string(),
        closes_in: "48h".to_string(),
        category: req.circle_id.clone(),
        tally: BallotTally {
            agree_pct: 100,
            oppose_pct: 0,
            neutral_pct: 0,
            agree_votes: 1,
            oppose_votes: 0,
            neutral_votes: 0,
            total_ballots: 1,
            quorum_pct: 4,
        },
        deliberation_count: 0,
        blake3_hash: audit_hash,
        top_dissent: None,
        created_at: chrono_now(),
    };

    engine.ballots.insert(0, new_ballot.clone());

    // Record receipt
    engine.receipts.insert(
        0,
        VerificationReceipt {
            tx_id: format!("#{seq}"),
            action: format!("Drafted Motion {code}"),
            block: format!("Block #{block}"),
            delta: "New Ballot Seed".to_string(),
            time_ago: "Just now".to_string(),
        },
    );

    Ok(Json(DraftMotionResponse {
        accepted: true,
        ballot: new_ballot,
    }))
}

fn chrono_now() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("{now}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pulse_telemetry_validity() {
        let pulse = get_pulse().await.0;
        assert!(pulse.block_height >= 849_201);
        assert_eq!(pulse.status, "ACTIVE");
        assert_eq!(pulse.network_label, "PRECINCT 42 NETWORK ACTIVE");
        assert_eq!(pulse.featured_circles.len(), 5);
        assert!(!pulse.blake3_anchor_hash.is_empty());
        assert_eq!(pulse.blake3_anchor_hash.len(), 64);
    }

    #[tokio::test]
    async fn test_ballots_list_and_filter() {
        let all = list_ballots(Query(BallotFilterQuery {
            category: None,
            status: None,
        }))
        .await
        .0;
        assert!(all.len() >= 3);

        let transit = list_ballots(Query(BallotFilterQuery {
            category: Some("transit".into()),
            status: None,
        }))
        .await
        .0;
        assert_eq!(transit.len(), 1);
        assert_eq!(transit[0].circle_id, "transit");
    }

    #[tokio::test]
    async fn test_vote_and_blake3_receipt() {
        let vote_res = cast_ballot_vote(Json(CastBallotRequest {
            ballot_id: "tc-2024-88".into(),
            choice: "agree".into(),
            voter_token: Some("citizen_test_user".into()),
        }))
        .await
        .expect("vote cast should succeed")
        .0;

        assert!(vote_res.accepted);
        assert_eq!(vote_res.choice, "agree");
        assert_eq!(vote_res.receipt_hash.len(), 64);
        assert!(vote_res.updated_tally.agree_votes > 2320);
    }

    #[tokio::test]
    async fn test_draft_motion() {
        let draft_res = draft_ballot_motion(Json(DraftMotionRequest {
            title: "Test Municipal Broadband Proposal".into(),
            summary: "Guarantee fiber broadband access across all district schools".into(),
            circle_id: "it-ethics".into(),
            author_name: Some("Aadarsh Pandey".into()),
        }))
        .await
        .expect("draft motion should succeed")
        .0;

        assert!(draft_res.accepted);
        assert_eq!(draft_res.ballot.author.name, "Aadarsh Pandey");
        assert_eq!(draft_res.ballot.author.avatar_initials, "AP");
        assert_eq!(draft_res.ballot.blake3_hash.len(), 64);
    }
}

