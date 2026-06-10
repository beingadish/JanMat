mod middleware;
mod routes;

use axum::{Json, Router, routing::get, routing::post};
use serde::Serialize;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "janmat-api",
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health))
        // Constitution
        .route("/pub/constitution", get(routes::governance::get_constitution))
        .route("/pub/proposals", get(routes::governance::list_proposals))
        // Public read routes
        .route("/pub/t", get(routes::namespace::list_topics))
        .route("/pub/t/:id", get(routes::namespace::get_topic))
        .route(
            "/pub/t/:topic_id/v/:vote_id/results",
            get(routes::votes::vote_results),
        )
        // Citizen write routes
        .route("/citizen/t", post(routes::namespace::create_topic))
        .route("/citizen/o", post(routes::opinions::submit_opinion))
        .route("/citizen/v", post(routes::votes::create_vote))
        .route("/citizen/v/:id/cast", post(routes::votes::cast_vote))
        .route("/citizen/v/:id/reveal", post(routes::votes::reveal_vote))
        .route("/citizen/proposals", post(routes::governance::create_proposal))
        .route("/citizen/recall/:topic_id", post(routes::governance::sign_recall));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("bind janmat-api listener");
    println!("janmat-api listening on 127.0.0.1:3000");
    axum::serve(listener, app)
        .await
        .expect("run janmat-api server");
}
