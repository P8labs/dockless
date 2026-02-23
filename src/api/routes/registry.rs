use crate::platform::node::Node;
use axum::{Json, Router, extract::State, response::IntoResponse, routing::get};

pub fn routes() -> Router<Node> {
    Router::new().route("/registry", get(get_registry))
}

async fn get_registry(State(node): State<Node>) -> impl IntoResponse {
    let registry = node.registry.read().await;
    Json(registry.list_definitions().to_vec())
}
