use anyhow::Context;
use axum::{
    Json, Router,
    http::Uri,
    response::{IntoResponse, Response},
};

use reqwest::{StatusCode, header};

use rust_embed::Embed;
use serde_json::json;
use std::net::SocketAddr;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;

use crate::{
    api::routes::{health, registry, services},
    platform::node::Node,
};

#[derive(Embed)]
#[folder = "portal/build/"]
struct PortalAssets;

pub async fn start_api(node: &Node) -> anyhow::Result<()> {
    let api_routes = Router::new()
        .merge(health::routes())
        .merge(services::routes())
        .merge(registry::routes())
        .with_state(node.clone());

    let app = Router::new()
        .nest("/api", api_routes.fallback(api_fallback))
        .fallback(serve_console)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

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

async fn api_fallback(uri: Uri) -> Response {
    (
        StatusCode::NOT_FOUND,
        Json(json!({
            "error": format!("Not found: {}", uri.path()),
        })),
    )
        .into_response()
}

async fn serve_console(uri: Uri) -> Response {
    let path = uri.path().trim_start_matches('/');

    if !path.is_empty() {
        if let Some(content) = PortalAssets::get(path) {
            let mime = mime_guess::from_path(path)
                .first_or_octet_stream()
                .to_string();
            return (
                StatusCode::OK,
                [(header::CONTENT_TYPE, mime)],
                content.data.to_vec(),
            )
                .into_response();
        }
    }

    if !path.is_empty() && !path.contains('.') {
        let html_path = format!("{}.html", path);
        if let Some(content) = PortalAssets::get(&html_path) {
            return (
                StatusCode::OK,
                [(header::CONTENT_TYPE, "text/html".to_string())],
                content.data.to_vec(),
            )
                .into_response();
        }
    }

    if !path.is_empty() {
        let index_path = format!("{}/index.html", path);
        if let Some(content) = PortalAssets::get(&index_path) {
            return (
                StatusCode::OK,
                [(header::CONTENT_TYPE, "text/html".to_string())],
                content.data.to_vec(),
            )
                .into_response();
        }
    }

    if let Some(content) = PortalAssets::get("index.html") {
        return (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "text/html".to_string())],
            content.data.to_vec(),
        )
            .into_response();
    }

    (
        StatusCode::NOT_FOUND,
        "Portal not available. Rebuild with: cargo clean && cargo build --release",
    )
        .into_response()
}

async fn shutdown_signal() {
    if let Err(e) = tokio::signal::ctrl_c().await {
        tracing::error!(error = ?e, "failed to listen for Ctrl+C");
    } else {
        tracing::info!("shutdown signal received");
    }
}
