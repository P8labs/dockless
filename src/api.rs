use std::net::SocketAddr;

use crate::{node::Node, service::ServiceState};
use anyhow::Context;
use axum::{Json, Router, extract::State, response::IntoResponse, routing::get};
use serde::Serialize;
use tracing::info;

#[derive(Serialize)]
struct ServiceInfo {
    id: String,
    name: String,
    state: ServiceState,
}

#[derive(Serialize)]
struct Health {
    name: String,
    status: String,
    node_id: String,
    services: Vec<ServiceInfo>,
}

pub async fn start_api(node: Node) -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(health_index))
        .route("/services", get(list_services))
        .with_state(node.clone());

    let addr = SocketAddr::from(([0, 0, 0, 0], node.config.listen_port));

    let listener = tokio::net::TcpListener::bind(addr).await.with_context(|| {
        format!(
            "failed to bind TCP listener on 0.0.0.0:{}",
            node.config.listen_port
        )
    })?;

    info!(
        "listening on {}",
        listener
            .local_addr()
            .context("failed to read local listener address")?
    );

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("HTTP server crashed unexpectedly")?;

    Ok(())
}

async fn shutdown_signal() {
    if let Err(e) = tokio::signal::ctrl_c().await {
        tracing::error!(error = ?e, "failed to listen for Ctrl+C");
    } else {
        tracing::info!("shutdown signal received");
    }
}

async fn health_index(State(node): State<Node>) -> impl IntoResponse {
    let mut services = Vec::new();

    for service in &node.services {
        let state = service.get_state().await;

        services.push(ServiceInfo {
            id: service.id.clone(),
            name: service.name.clone(),
            state,
        });
    }

    let health = Health {
        name: "dockless".to_string(),
        status: "alive".to_string(),
        node_id: node.node_id.clone(),
        services,
    };

    Json(health)
}

async fn list_services(State(node): State<Node>) -> impl IntoResponse {
    let mut services = Vec::new();

    for service in &node.services {
        let state = service.get_state().await;

        services.push(ServiceInfo {
            id: service.id.clone(),
            name: service.name.clone(),
            state,
        });
    }

    Json(services)
}
