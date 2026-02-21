use std::net::SocketAddr;

use crate::{
    node::Node,
    registry::ServiceDefinition,
    service::{Service, ServiceState},
};
use anyhow::Context;
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
};
use serde::Serialize;
use serde_json::json;
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
}

pub async fn start_api(node: &Node) -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(health_index))
        .route("/services", get(list_services))
        .route("/services", post(create_service))
        .route("/services/{id}", delete(delete_service))
        .route("/services/{id}/start", post(start_service))
        .route("/services/{id}/stop", post(stop_service))
        .route("/services/{id}/restart", post(restart_service))
        .route("/registry", get(get_registry))
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
    let health = Health {
        name: "dockless".to_string(),
        status: "alive".to_string(),
        node_id: node.node_id.clone(),
    };

    Json(health)
}

async fn list_services(State(node): State<Node>) -> impl IntoResponse {
    let mut services = Vec::new();

    for service in node.services.read().await.iter() {
        let state = service.get_state().await;

        services.push(ServiceInfo {
            id: service.id.clone(),
            name: service.name.clone(),
            state,
        });
    }

    Json(services)
}

async fn start_service(State(node): State<Node>, Path(id): Path<String>) -> impl IntoResponse {
    if let Some(service) = node.services.read().await.iter().find(|s| s.id == id) {
        let mut manager = node.manager.write().await;

        match manager.start(&service).await {
            Ok(_) => (
                StatusCode::OK,
                Json(json!({
                    "status": true,
                    "message": "Service started"
                })),
            )
                .into_response(),

            Err(e) => (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "status": false,
                    "error": e.to_string()
                })),
            )
                .into_response(),
        }
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(json!({
                "status": false,
                "message": "service not found"
            })),
        )
            .into_response()
    }
}

async fn stop_service(State(node): State<Node>, Path(id): Path<String>) -> impl IntoResponse {
    if let Some(service) = node.services.read().await.iter().find(|s| s.id == id) {
        let mut manager = node.manager.write().await;

        match manager.stop(&service.id).await {
            Ok(_) => (
                StatusCode::OK,
                Json(json!({
                    "status": true,
                    "message": "Service stopped"
                })),
            )
                .into_response(),

            Err(e) => (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "status": false,
                    "error": e.to_string()
                })),
            )
                .into_response(),
        }
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(json!({
                "status": false,
                "message": "service not found"
            })),
        )
            .into_response()
    }
}

async fn restart_service(State(node): State<Node>, Path(id): Path<String>) -> impl IntoResponse {
    if let Some(service) = node.services.read().await.iter().find(|s| s.id == id) {
        let mut manager = node.manager.write().await;

        match manager.restart(&service).await {
            Ok(_) => (
                StatusCode::OK,
                Json(json!({
                    "status": true,
                    "message": "Service restarted"
                })),
            )
                .into_response(),

            Err(e) => (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "status": false,
                    "error": e.to_string()
                })),
            )
                .into_response(),
        }
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(json!({
                "status": false,
                "message": "service not found"
            })),
        )
            .into_response()
    }
}

async fn create_service(
    State(node): State<Node>,
    Json(def): Json<ServiceDefinition>,
) -> impl IntoResponse {
    if def.id.trim().is_empty() || def.binary_path.trim().is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "status": false,
                "error": "id and binary_path are required"
            })),
        )
            .into_response();
    }

    {
        let mut registry = node.registry.write().await;

        if let Err(e) = registry.add(def.clone()) {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "status": false,
                    "error": e.to_string()
                })),
            )
                .into_response();
        }

        if let Err(e) = registry.save() {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": false,
                    "error": e.to_string()
                })),
            )
                .into_response();
        }
    }

    let service = Service::new(
        def.id.clone(),
        def.name.clone(),
        def.binary_path.clone(),
        def.args.clone(),
        def.env.clone(),
        def.auto_restart,
        def.restart_limit,
    );

    {
        let mut services = node.services.write().await;
        services.push(service.clone());
    }

    {
        let mut manager = node.manager.write().await;

        if let Err(e) = manager.start(&service).await {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": false,
                    "error": e.to_string()
                })),
            )
                .into_response();
        }
    }

    (
        StatusCode::CREATED,
        Json(json!({
            "status": true,
            "message": "Service created and started"
        })),
    )
        .into_response()
}

async fn delete_service(State(node): State<Node>, Path(id): Path<String>) -> impl IntoResponse {
    {
        let registry = node.registry.read().await;
        if !registry.list_definitions().iter().any(|s| s.id == id) {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "status": false,
                    "error": "service not found"
                })),
            )
                .into_response();
        }
    }

    {
        let mut manager = node.manager.write().await;
        let _ = manager.stop(&id).await;
    }

    {
        let mut registry = node.registry.write().await;

        if let Err(e) = registry.remove(&id) {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "status": false,
                    "error": e.to_string()
                })),
            )
                .into_response();
        }

        if let Err(e) = registry.save() {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": false,
                    "error": e.to_string()
                })),
            )
                .into_response();
        }
    }

    {
        let mut services = node.services.write().await;
        services.retain(|s| s.id != id);
    }

    (
        StatusCode::OK,
        Json(json!({
            "status": true,
            "message": "Service deleted"
        })),
    )
        .into_response()
}

async fn get_registry(State(node): State<Node>) -> impl IntoResponse {
    let registry = node.registry.read().await;
    let df = registry.list_definitions().to_owned();
    Json(df)
}
