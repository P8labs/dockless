use crate::node::Node;
use axum::{Json, Router, extract::State, response::IntoResponse, routing::get};
use serde::Serialize;

#[derive(Serialize)]
struct Health {
    name: String,
    status: String,
    node_id: String,
}
pub async fn start_api(node: Node) {
    let app = Router::new().route("/", get(health_index)).with_state(node);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    let _ = axum::serve(listener, app).await;
}

async fn health_index(State(node): State<Node>) -> impl IntoResponse {
    let health: Health = Health {
        name: "dockless".to_string(),
        status: "alive".to_string(),
        node_id: node.node_id,
    };
    Json(health)
}
