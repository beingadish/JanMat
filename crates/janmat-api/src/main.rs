mod db;
mod middleware;
mod routes;

use std::sync::Arc;

use axum::{
    Json, Router,
    body::Body,
    extract::Request,
    http::{HeaderValue, Method, Response, StatusCode},
    middleware::Next,
    routing::{get, post},
};
use serde::Serialize;

use db::Registry;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
    version: &'static str,
    network: &'static str,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "janmat-api",
        version: "0.2.0",
        network: "precinct-42-active",
    })
}

async fn cors(req: Request, next: Next) -> Response<Body> {
    if req.method() == Method::OPTIONS {
        let mut response = Response::new(Body::empty());
        *response.status_mut() = StatusCode::NO_CONTENT;
        let headers = response.headers_mut();
        headers.insert("access-control-allow-origin", HeaderValue::from_static("*"));
        headers.insert(
            "access-control-allow-methods",
            HeaderValue::from_static("GET, POST, PUT, DELETE, OPTIONS"),
        );
        headers.insert(
            "access-control-allow-headers",
            HeaderValue::from_static("*"),
        );
        return response;
    }

    let mut response = next.run(req).await;
    let headers = response.headers_mut();
    headers.insert("access-control-allow-origin", HeaderValue::from_static("*"));
    headers.insert(
        "access-control-allow-methods",
        HeaderValue::from_static("GET, POST, PUT, DELETE, OPTIONS"),
    );
    headers.insert(
        "access-control-allow-headers",
        HeaderValue::from_static("*"),
    );
    response
}

#[tokio::main]
async fn main() {
    // Citizen registry — Postgres if DATABASE_URL is reachable, else in-memory.
    let registry: Arc<Registry> = Registry::init(std::env::var("DATABASE_URL").ok()).await;

    let app: Router = Router::new()
        .route("/health", get(health))
        .route("/api/health", get(health))
        // Real-Time Civic Pulse & Verification
        .route("/pub/pulse", get(routes::pulse::get_pulse))
        .route("/api/pulse", get(routes::pulse::get_pulse))
        .route("/pub/ballots", get(routes::pulse::list_ballots))
        .route("/api/ballots", get(routes::pulse::list_ballots))
        .route("/pub/ballots/{id}", get(routes::pulse::get_ballot))
        .route("/api/ballots/{id}", get(routes::pulse::get_ballot))
        .route("/citizen/ballots/vote", post(routes::pulse::cast_ballot_vote))
        .route("/api/citizen/ballots/vote", post(routes::pulse::cast_ballot_vote))
        .route("/citizen/ballots/draft", post(routes::pulse::draft_ballot_motion))
        .route("/api/citizen/ballots/draft", post(routes::pulse::draft_ballot_motion))
        // Constitution
        .route(
            "/pub/constitution",
            get(routes::governance::get_constitution),
        )
        .route("/pub/proposals", get(routes::governance::list_proposals))
        // Public read routes
        .route("/pub/t", get(routes::namespace::list_topics))
        .route("/pub/t/{id}", get(routes::namespace::get_topic))
        .route(
            "/pub/t/{topic_id}/v/{vote_id}/results",
            get(routes::votes::vote_results),
        )
        // Identity: POC registration / login (no external verification)
        .route("/citizen/register", post(routes::register::register))
        .route("/citizen/login", post(routes::register::login))
        .route("/citizen/password", post(routes::register::regenerate))
        .route(
            "/citizen/username/{name}",
            get(routes::register::username_available),
        )
        // KYC routes (real UIDAI flow, kept for completeness)
        .route("/citizen/kyc/health", get(routes::kyc::kyc_health))
        .route("/citizen/kyc/upload", post(routes::kyc::upload_kyc))
        // Citizen write routes
        .route("/citizen/t", post(routes::namespace::create_topic))
        .route("/citizen/o", post(routes::opinions::submit_opinion))
        .route("/citizen/v", post(routes::votes::create_vote))
        .route("/citizen/v/{id}/cast", post(routes::votes::cast_vote))
        .route("/citizen/v/{id}/reveal", post(routes::votes::reveal_vote))
        .route(
            "/citizen/proposals",
            post(routes::governance::create_proposal),
        )
        .route(
            "/citizen/recall/{topic_id}",
            post(routes::governance::sign_recall),
        )
        .layer(axum::middleware::from_fn(cors))
        .with_state(registry);

    // Bind address is configurable so the same binary works locally
    // (127.0.0.1:3000) and inside a container (0.0.0.0:3000 via JANMAT_BIND_ADDR).
    let bind_addr =
        std::env::var("JANMAT_BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_owned());
    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .expect("bind janmat-api listener");
    println!("janmat-api listening on {bind_addr}");
    axum::serve(listener, app)
        .await
        .expect("run janmat-api server");
}
