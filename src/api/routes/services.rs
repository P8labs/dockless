use crate::platform::node::Node;
use axum::extract::{Multipart, Path, State};
use axum::{
    Json, Router,
    http::StatusCode,
    response::{IntoResponse, Sse},
    routing::{delete, get, post},
};
use serde_json::json;
use std::collections::HashMap;
use std::convert::Infallible;
use std::fs;
use std::time::Duration;
use sysinfo::System;

use serde::{Deserialize, Serialize};

use crate::{
    registry::ServiceDefinition,
    runtime::service::{Service, ServiceState},
};

pub fn routes() -> Router<Node> {
    Router::new()
        .route("/services", get(list_services))
        .route("/services", post(create_service))
        .route("/services/init", post(init_service))
        .route("/services/{id}", get(get_service))
        .route("/services/{id}", delete(delete_service))
        .route("/services/{id}/configure", post(configure_service))
        .route("/services/{id}/start", post(start_service))
        .route("/services/{id}/stop", post(stop_service))
        .route("/services/{id}/restart", post(restart_service))
        .route("/services/{id}/artifact/upload", post(upload_artifact))
        .route(
            "/services/{id}/artifact/github",
            post(install_github_artifact),
        )
        .route("/services/{id}/artifact", get(get_artifact_info))
        .route("/services/{id}/config", get(get_service_config))
        .route("/services/{id}/config", post(update_service_config))
        .route(
            "/services/{id}/config/template",
            post(create_or_update_template),
        )
        .route(
            "/services/{id}/config/template",
            delete(delete_config_template),
        )
        .route("/services/ports", get(get_port_allocations))
        .route("/services/{id}/logs", get(get_logs))
        .route("/services/{id}/logs/stream", get(stream_logs))
        .route("/services/{id}/logs/clear", post(clear_logs))
        .route("/services/{id}/stats", get(get_service_stats))
}

#[derive(Serialize)]
struct ServiceInfo {
    id: String,
    name: String,
    state: ServiceState,
    ready: bool,
}
async fn list_services(State(node): State<Node>) -> impl IntoResponse {
    let mut services = Vec::new();

    let registry = node.registry.read().await;

    for service in node.manager.read().await.list().await.iter() {
        let state = service.get_state().await;
        let def = registry.get(&service.id);
        let ready = def.map(|d| d.ready).unwrap_or(false);

        services.push(ServiceInfo {
            id: service.id.clone(),
            name: service.name.clone(),
            state,
            ready,
        });
    }

    Json(services)
}

#[derive(Deserialize)]
struct InitServiceRequest {
    name: String,
    #[serde(default)]
    id: Option<String>,
}

async fn init_service(
    State(node): State<Node>,
    Json(req): Json<InitServiceRequest>,
) -> impl IntoResponse {
    if req.name.trim().is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "status": false,
                "error": "name is required"
            })),
        )
            .into_response();
    }

    let id = req.id.unwrap_or_else(|| {
        req.name
            .to_lowercase()
            .replace(char::is_whitespace, "-")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-')
            .collect()
    });

    let service_root = format!("{}/services/{}", node.config.data_dir, id);
    let bin_dir = format!("{}/bin", service_root);
    let data_dir = format!("{}/data", service_root);
    let logs_dir = format!("{}/logs", service_root);
    let versions_dir = format!("{}/versions", service_root);

    if let Err(e) = std::fs::create_dir_all(&bin_dir)
        .and_then(|_| std::fs::create_dir_all(&data_dir))
        .and_then(|_| std::fs::create_dir_all(&logs_dir))
        .and_then(|_| std::fs::create_dir_all(&versions_dir))
    {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "status": false,
                "error": format!("failed to create service directories: {}", e)
            })),
        )
            .into_response();
    }

    let def = ServiceDefinition {
        id: id.clone(),
        name: req.name.clone(),
        ready: false,
        binary_path: String::new(),
        args: vec![],
        env: HashMap::new(),
        auto_restart: true,
        restart_limit: None,
        current_version: None,
        port: None,
    };

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

    {
        let mut port_manager = node.port_manager.write().await;
        if let Err(e) = port_manager.allocate(&id) {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": false,
                    "error": format!("failed to allocate port: {}", e)
                })),
            )
                .into_response();
        }
    }

    (
        StatusCode::CREATED,
        Json(json!({
            "status": true,
            "message": "Service initialized",
            "id": id
        })),
    )
        .into_response()
}

async fn get_service(State(node): State<Node>, Path(id): Path<String>) -> impl IntoResponse {
    let def = {
        let registry = node.registry.read().await;
        match registry.get(&id) {
            Some(d) => d.clone(),
            None => {
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
    };

    let port = {
        let port_manager = node.port_manager.read().await;
        port_manager.get_port(&id)
    };

    let mut response = json!({
        "id": def.id,
        "name": def.name,
        "ready": def.ready,
        "binary_path": def.binary_path,
        "args": def.args,
        "env": def.env,
        "auto_restart": def.auto_restart,
        "restart_limit": def.restart_limit,
        "current_version": def.current_version,
    });

    if let Some(port_num) = port {
        response["port"] = json!(port_num);
    }

    (StatusCode::OK, Json(response)).into_response()
}

#[derive(Deserialize)]
pub struct ConfigureServiceRequest {
    #[serde(default)]
    pub env: HashMap<String, String>,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub auto_restart: Option<bool>,
    #[serde(default)]
    pub restart_limit: Option<u32>,
}

async fn configure_service(
    State(node): State<Node>,
    Path(id): Path<String>,
    Json(req): Json<ConfigureServiceRequest>,
) -> impl IntoResponse {
    let mut registry = node.registry.write().await;

    let def = match registry.get(&id) {
        Some(d) => d.clone(),
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "status": false,
                    "error": "service not found"
                })),
            )
                .into_response();
        }
    };

    let updated_def = ServiceDefinition {
        env: req.env,
        args: req.args,
        auto_restart: req.auto_restart.unwrap_or(def.auto_restart),
        restart_limit: req.restart_limit,
        ..def
    };

    if let Err(e) = registry.update(&id, updated_def) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
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

    (
        StatusCode::OK,
        Json(json!({
            "status": true,
            "message": "Service configured"
        })),
    )
        .into_response()
}

async fn start_service(State(node): State<Node>, Path(id): Path<String>) -> impl IntoResponse {
    {
        let registry = node.registry.read().await;
        if let Some(def) = registry.get(&id) {
            if !def.ready {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "status": false,
                        "error": "Service is not ready. Please upload a binary first."
                    })),
                )
                    .into_response();
            }
        } else {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "status": false,
                    "error": "Service not found"
                })),
            )
                .into_response();
        }
    }

    let service_exists = node
        .manager
        .read()
        .await
        .list_ids()
        .await
        .iter()
        .any(|s| s == &id);

    if service_exists {
        let mut manager = node.manager.write().await;

        match manager.start(&id).await {
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
                "error": "Service not found"
            })),
        )
            .into_response()
    }
}

async fn stop_service(State(node): State<Node>, Path(id): Path<String>) -> impl IntoResponse {
    {
        let registry = node.registry.read().await;
        if let Some(def) = registry.get(&id) {
            if !def.ready {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "status": false,
                        "error": "Service is not ready. Please upload a binary first."
                    })),
                )
                    .into_response();
            }
        } else {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "status": false,
                    "error": "Service not found"
                })),
            )
                .into_response();
        }
    }

    let service_exists = node
        .manager
        .read()
        .await
        .list_ids()
        .await
        .iter()
        .any(|s| s == &id);

    if service_exists {
        let mut manager = node.manager.write().await;

        match manager.stop(&id).await {
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
                "error": "Service not found"
            })),
        )
            .into_response()
    }
}

async fn restart_service(State(node): State<Node>, Path(id): Path<String>) -> impl IntoResponse {
    {
        let registry = node.registry.read().await;
        if let Some(def) = registry.get(&id) {
            if !def.ready {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "status": false,
                        "error": "Service is not ready. Please upload a binary first."
                    })),
                )
                    .into_response();
            }
        } else {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "status": false,
                    "error": "Service not found"
                })),
            )
                .into_response();
        }
    }

    let service_exists = node
        .manager
        .read()
        .await
        .list_ids()
        .await
        .iter()
        .any(|s| s == &id);

    if service_exists {
        let mut manager = node.manager.write().await;

        match manager.restart(&id).await {
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
                "error": "Service not found"
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

    let binary_path = std::path::Path::new(&def.binary_path);

    if !binary_path.is_absolute() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "status": false,
                "error": "binary_path must be absolute"
            })),
        )
            .into_response();
    }

    if !binary_path.exists() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "status": false,
                "error": "binary_path does not exist"
            })),
        )
            .into_response();
    }

    let service_root = format!("{}/services/{}", node.config.data_dir, def.id);
    let bin_dir = format!("{}/bin", service_root);
    let data_dir = format!("{}/data", service_root);
    let logs_dir = format!("{}/logs", service_root);

    if let Err(e) = std::fs::create_dir_all(&bin_dir)
        .and_then(|_| std::fs::create_dir_all(&data_dir))
        .and_then(|_| std::fs::create_dir_all(&logs_dir))
    {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "status": false,
                "error": format!("failed to create service directories: {}", e)
            })),
        )
            .into_response();
    }

    let port = {
        let mut port_manager = node.port_manager.write().await;
        match port_manager.allocate(&def.id) {
            Ok(p) => p,
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "status": false,
                        "error": format!("failed to allocate port: {}", e)
                    })),
                )
                    .into_response();
            }
        }
    };

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

    let mut env = def.env.clone();
    env.insert("PORT".to_string(), port.to_string());

    let service = Service::new(
        def.id.clone(),
        def.name.clone(),
        def.binary_path.clone(),
        def.args.clone(),
        env,
        def.auto_restart,
        def.restart_limit.or(Some(3)),
        service_root.clone(),
    );

    {
        let mut manager = node.manager.write().await;
        let _ = manager.register_service(service.clone());

        if let Err(e) = manager.start(&service.id).await {
            let mut registry = node.registry.write().await;
            let _ = registry.remove(&def.id);
            let _ = registry.save();

            let mut port_manager = node.port_manager.write().await;
            let _ = port_manager.deallocate(&def.id);

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
        let mut port_manager = node.port_manager.write().await;
        let _ = port_manager.deallocate(&id);
    }

    {
        let mut manager = node.manager.write().await;
        let _ = manager.stop(&id).await;
        let _ = manager.unregister_service(&id);
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

    let service_root = format!("{}/services/{}", node.config.data_dir, id);
    let _ = std::fs::remove_dir_all(&service_root);

    (
        StatusCode::OK,
        Json(json!({
            "status": true,
            "message": "Service deleted"
        })),
    )
        .into_response()
}

pub async fn upload_artifact(
    State(node): State<Node>,
    Path(id): Path<String>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    {
        let registry = node.registry.read().await;
        if !registry.list_definitions().iter().any(|s| s.id == id) {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({"status": false, "error": "service not found"})),
            )
                .into_response();
        }
    }

    let mut version: Option<String> = None;
    let mut file_name: Option<String> = None;
    let mut file_bytes: Option<Vec<u8>> = None;

    while let Ok(Some(field)) = multipart.next_field().await {
        match field.name() {
            Some("version") => {
                version = Some(field.text().await.unwrap_or_default());
            }
            Some("file") => {
                file_name = field.file_name().map(|s| s.to_string());
                file_bytes = Some(field.bytes().await.unwrap().to_vec());
            }
            _ => {}
        }
    }

    if version.is_none() || file_name.is_none() || file_bytes.is_none() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"status": false, "error": "version and file are required"})),
        )
            .into_response();
    }

    let version = version.unwrap();
    let file_name = file_name.unwrap();
    let file_bytes = file_bytes.unwrap();

    let service_root = format!("{}/services/{}", node.config.data_dir, id);
    let version_dir = format!("{}/versions/{}", service_root, version);
    let bin_dir = format!("{}/bin", service_root);

    if let Err(e) = fs::create_dir_all(&version_dir) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": false, "error": e.to_string()})),
        )
            .into_response();
    }

    let binary_path = format!("{}/{}", version_dir, file_name);

    if let Err(e) = fs::write(&binary_path, file_bytes) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": false, "error": e.to_string()})),
        )
            .into_response();
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(&binary_path, fs::Permissions::from_mode(0o755));
    }

    let existing_binary_name = {
        let registry = node.registry.read().await;
        registry
            .get(&id)
            .filter(|def| def.ready && !def.binary_path.is_empty())
            .and_then(|def| def.binary_path.split('/').last().map(|s| s.to_string()))
    };

    let final_binary_name = if let Some(existing_name) = existing_binary_name {
        if existing_name != file_name {
            let consistent_path = format!("{}/{}", version_dir, existing_name);
            if let Err(e) = fs::copy(&binary_path, &consistent_path) {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(
                        json!({"status": false, "error": format!("failed to copy binary: {}", e)}),
                    ),
                )
                    .into_response();
            }
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let _ = fs::set_permissions(&consistent_path, fs::Permissions::from_mode(0o755));
            }
        }
        existing_name
    } else {
        file_name.clone()
    };

    let current_link = format!("{}/current", bin_dir);
    let _ = fs::remove_file(&current_link);

    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        let relative_target = format!("../versions/{}", version);
        if let Err(e) = symlink(&relative_target, &current_link) {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": false, "error": e.to_string()})),
            )
                .into_response();
        }
    }

    {
        let mut registry = node.registry.write().await;

        if let Some(def) = registry
            .list_definitions_mut()
            .iter_mut()
            .find(|s| s.id == id)
        {
            def.current_version = Some(version.clone());

            if def.binary_path.is_empty() || !def.ready {
                let binary_path_relative = format!("bin/current/{}", final_binary_name);
                def.binary_path = binary_path_relative;
            }
            def.ready = true;
        }

        let _ = registry.save();
    }

    let service_to_register = {
        let manager = node.manager.read().await;
        let service_exists = manager.list_ids().await.iter().any(|s| s == &id);
        drop(manager);

        let registry = node.registry.read().await;
        if let Some(def) = registry.get(&id) {
            let service_root = format!("{}/services/{}", node.config.data_dir, id);
            let mut env = def.env.clone();

            let port_manager = node.port_manager.read().await;
            if let Some(port) = port_manager.get_port(&id) {
                env.insert("PORT".to_string(), port.to_string());
            }
            drop(port_manager);

            let service = Service::new(
                def.id.clone(),
                def.name.clone(),
                def.binary_path.clone(),
                def.args.clone(),
                env,
                def.auto_restart,
                def.restart_limit,
                service_root,
            );
            drop(registry);

            Some((service, service_exists))
        } else {
            drop(registry);
            None
        }
    };

    if let Some((service, exists)) = service_to_register {
        let mut manager = node.manager.write().await;
        if exists {
            let _ = manager.update_service(service);
        } else {
            let _ = manager.register_service(service);
        }

        match tokio::time::timeout(tokio::time::Duration::from_secs(30), manager.restart(&id)).await
        {
            Ok(Ok(_)) => {}
            Ok(Err(e)) => tracing::error!("restart failed: {}", e),
            Err(_) => tracing::error!("restart timed out after 30s"),
        }
        drop(manager);
    }

    (
        StatusCode::OK,
        Json(json!({"status": true, "message": "artifact uploaded"})),
    )
        .into_response()
}

#[derive(Deserialize)]
pub struct GithubArtifactRequest {
    pub repo: String,
    pub version: String,
    pub asset: String,
}

pub async fn install_github_artifact(
    State(node): State<Node>,
    Path(id): Path<String>,
    Json(payload): Json<GithubArtifactRequest>,
) -> impl IntoResponse {
    {
        let registry = node.registry.read().await;
        if !registry.list_definitions().iter().any(|s| s.id == id) {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({"status": false, "error": "service not found"})),
            )
                .into_response();
        }
    }

    let url = format!(
        "https://api.github.com/repos/{}/releases/tags/{}",
        payload.repo, payload.version
    );

    let client = reqwest::Client::new();

    let response = match client
        .get(&url)
        .header("User-Agent", "dockless")
        .send()
        .await
    {
        Ok(r) => r,
        Err(e) => {
            return (
                StatusCode::BAD_GATEWAY,
                Json(json!({"status": false, "error": e.to_string()})),
            )
                .into_response();
        }
    };

    if !response.status().is_success() {
        return (
            StatusCode::BAD_GATEWAY,
            Json(json!({"status": false, "error": "failed to fetch release"})),
        )
            .into_response();
    }

    let release_json: serde_json::Value = match response.json().await {
        Ok(v) => v,
        Err(e) => {
            return (
                StatusCode::BAD_GATEWAY,
                Json(json!({"status": false, "error": e.to_string()})),
            )
                .into_response();
        }
    };

    let empty_vec = vec![];
    let assets = release_json["assets"].as_array().unwrap_or(&empty_vec);

    let asset = assets
        .iter()
        .find(|a| a["name"].as_str() == Some(&payload.asset));

    if asset.is_none() {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({"status": false, "error": "asset not found"})),
        )
            .into_response();
    }

    let download_url = asset.unwrap()["browser_download_url"]
        .as_str()
        .unwrap_or("");

    if download_url.is_empty() {
        return (
            StatusCode::BAD_GATEWAY,
            Json(json!({"status": false, "error": "invalid asset url"})),
        )
            .into_response();
    }

    let response = match client
        .get(download_url)
        .header("User-Agent", "dockless")
        .send()
        .await
    {
        Ok(r) => r,
        Err(e) => {
            return (
                StatusCode::BAD_GATEWAY,
                Json(json!({"status": false, "error": e.to_string()})),
            )
                .into_response();
        }
    };

    let binary_bytes = match response.bytes().await {
        Ok(b) => b,
        Err(e) => {
            return (
                StatusCode::BAD_GATEWAY,
                Json(json!({"status": false, "error": e.to_string()})),
            )
                .into_response();
        }
    };

    let service_root = format!("{}/services/{}", node.config.data_dir, id);
    let version_dir = format!("{}/versions/{}", service_root, payload.version);
    let bin_dir = format!("{}/bin", service_root);

    if let Err(e) = std::fs::create_dir_all(&version_dir) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": false, "error": e.to_string()})),
        )
            .into_response();
    }

    let binary_path = format!("{}/{}", version_dir, payload.asset);

    if let Err(e) = std::fs::write(&binary_path, &binary_bytes) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": false, "error": e.to_string()})),
        )
            .into_response();
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(&binary_path, std::fs::Permissions::from_mode(0o755));
    }

    let existing_binary_name = {
        let registry = node.registry.read().await;
        registry
            .get(&id)
            .filter(|def| def.ready && !def.binary_path.is_empty())
            .and_then(|def| def.binary_path.split('/').last().map(|s| s.to_string()))
    };

    let final_binary_name = if let Some(existing_name) = existing_binary_name {
        if existing_name != payload.asset {
            let consistent_path = format!("{}/{}", version_dir, existing_name);
            if let Err(e) = std::fs::copy(&binary_path, &consistent_path) {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(
                        json!({"status": false, "error": format!("failed to copy binary: {}", e)}),
                    ),
                )
                    .into_response();
            }
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let _ = std::fs::set_permissions(
                    &consistent_path,
                    std::fs::Permissions::from_mode(0o755),
                );
            }
        }
        existing_name
    } else {
        payload.asset.clone()
    };

    let current_link = format!("{}/current", bin_dir);
    let _ = std::fs::remove_file(&current_link);

    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        let relative_target = format!("../versions/{}", payload.version);
        if let Err(e) = symlink(&relative_target, &current_link) {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": false, "error": e.to_string()})),
            )
                .into_response();
        }
    }

    {
        let mut registry = node.registry.write().await;

        if let Some(def) = registry
            .list_definitions_mut()
            .iter_mut()
            .find(|s| s.id == id)
        {
            def.current_version = Some(payload.version.clone());

            if def.binary_path.is_empty() || !def.ready {
                let binary_path_relative = format!("bin/current/{}", final_binary_name);
                def.binary_path = binary_path_relative;
            }
            def.ready = true;
        }

        let _ = registry.save();
    }

    let service_to_register = {
        let manager = node.manager.read().await;
        let service_exists = manager.list_ids().await.iter().any(|s| s == &id);
        drop(manager);

        let registry = node.registry.read().await;
        if let Some(def) = registry.get(&id) {
            let service_root = format!("{}/services/{}", node.config.data_dir, id);
            let mut env = def.env.clone();

            let port_manager = node.port_manager.read().await;
            if let Some(port) = port_manager.get_port(&id) {
                env.insert("PORT".to_string(), port.to_string());
            }
            drop(port_manager);

            let service = Service::new(
                def.id.clone(),
                def.name.clone(),
                def.binary_path.clone(),
                def.args.clone(),
                env,
                def.auto_restart,
                def.restart_limit,
                service_root,
            );
            drop(registry);

            Some((service, service_exists))
        } else {
            drop(registry);
            None
        }
    };

    if let Some((service, exists)) = service_to_register {
        let mut manager = node.manager.write().await;
        if exists {
            let _ = manager.update_service(service);
        } else {
            let _ = manager.register_service(service);
        }

        match tokio::time::timeout(tokio::time::Duration::from_secs(30), manager.restart(&id)).await
        {
            Ok(Ok(_)) => {}
            Ok(Err(e)) => tracing::error!("restart failed: {}", e),
            Err(_) => tracing::error!("restart timed out after 30s"),
        }
        drop(manager);
    }

    (
        StatusCode::OK,
        Json(json!({"status": true, "message": "github artifact installed"})),
    )
        .into_response()
}

pub async fn get_artifact_info(
    State(node): State<Node>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let service_root = format!("{}/services/{}", node.config.data_dir, id);
    let versions_dir = format!("{}/versions", service_root);

    let mut versions = vec![];

    if let Ok(entries) = std::fs::read_dir(&versions_dir) {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                if let Some(name) = entry.file_name().to_str() {
                    versions.push(name.to_string());
                }
            }
        }
    }

    let current_version = {
        let registry = node.registry.read().await;
        registry
            .list_definitions()
            .iter()
            .find(|s| s.id == id)
            .and_then(|s| s.current_version.clone())
    };

    Json(json!({
        "service": id,
        "current_version": current_version,
        "available_versions": versions
    }))
}

#[derive(Serialize)]
struct ConfigField {
    key: String,
    value: String,
    field_type: String,
    description: String,
}

#[derive(Serialize)]
struct ServiceConfig {
    has_config: bool,
    has_template: bool,
    fields: Vec<ConfigField>,
}

pub async fn get_service_config(
    State(node): State<Node>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    {
        let registry = node.registry.read().await;
        if !registry.list_definitions().iter().any(|s| s.id == id) {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({"status": false, "error": "service not found"})),
            )
                .into_response();
        }
    }

    let service_root = format!("{}/services/{}", node.config.data_dir, id);
    let config_path = format!("{}/config.toml", service_root);
    let template_path = format!("{}/config.example.toml", service_root);

    let has_template = std::path::Path::new(&template_path).exists();
    let has_config = std::path::Path::new(&config_path).exists();

    let mut fields = Vec::new();
    if has_template {
        if let Ok(template_content) = fs::read_to_string(&template_path) {
            if let Ok(template_toml) = toml::from_str(&template_content) {
                let current_config = if has_config {
                    fs::read_to_string(&config_path)
                        .ok()
                        .and_then(|c| toml::from_str(&c).ok())
                } else {
                    None
                };

                fields = extract_config_fields(&template_toml, current_config.as_ref());
            }
        }
    } else if has_config {
        if let Ok(config_content) = fs::read_to_string(&config_path) {
            if let Ok(config_toml) = toml::from_str(&config_content) {
                fn flatten(
                    prefix: Option<String>,
                    value: &toml::Value,
                    out: &mut Vec<ConfigField>,
                ) {
                    if let Some(table) = value.as_table() {
                        for (k, v) in table {
                            let key = match &prefix {
                                Some(p) => format!("{}.{}", p, k),
                                None => k.clone(),
                            };
                            match v {
                                toml::Value::Table(_) => flatten(Some(key), v, out),
                                _ => {
                                    let field_type = match v {
                                        toml::Value::String(_) => "string",
                                        toml::Value::Integer(_) => "integer",
                                        toml::Value::Float(_) => "float",
                                        toml::Value::Boolean(_) => "boolean",
                                        _ => "string",
                                    }
                                    .to_string();
                                    out.push(ConfigField {
                                        key,
                                        value: v.to_string().trim_matches('"').to_string(),
                                        field_type,
                                        description: String::new(),
                                    });
                                }
                            }
                        }
                    }
                }

                flatten(None, &config_toml, &mut fields);
            }
        }
    }

    Json(json!(ServiceConfig {
        has_config,
        has_template,
        fields,
    }))
    .into_response()
}

fn extract_config_fields(
    template: &toml::Value,
    current: Option<&toml::Value>,
) -> Vec<ConfigField> {
    let mut fields = Vec::new();

    if let Some(table) = template.as_table() {
        for (key, value) in table {
            let description = format!("Configuration for {}", key);
            if key.contains('.') {
                let field_type = match value {
                    toml::Value::String(_) => "string",
                    toml::Value::Integer(_) => "integer",
                    toml::Value::Float(_) => "float",
                    toml::Value::Boolean(_) => "boolean",
                    _ => "string",
                }
                .to_string();
                let current_value = get_value_by_path(current, &key)
                    .map(|v| v.to_string().trim_matches('"').to_string())
                    .unwrap_or_else(|| value.to_string().trim_matches('"').to_string());
                fields.push(ConfigField {
                    key: key.clone(),
                    value: current_value,
                    field_type,
                    description,
                });
            } else {
                match value {
                    toml::Value::Table(_) => {
                        let nested_fields = extract_nested_fields(key, value, current);
                        fields.extend(nested_fields);
                    }
                    _ => {
                        let field_type = match value {
                            toml::Value::String(_) => "string",
                            toml::Value::Integer(_) => "integer",
                            toml::Value::Float(_) => "float",
                            toml::Value::Boolean(_) => "boolean",
                            _ => "string",
                        }
                        .to_string();
                        let current_value = get_value_by_path(current, key)
                            .map(|v| v.to_string().trim_matches('"').to_string())
                            .unwrap_or_else(|| value.to_string().trim_matches('"').to_string());
                        fields.push(ConfigField {
                            key: key.clone(),
                            value: current_value,
                            field_type,
                            description,
                        });
                    }
                }
            }
        }
    }

    fields
}

fn extract_nested_fields(
    parent_key: &str,
    value: &toml::Value,
    current: Option<&toml::Value>,
) -> Vec<ConfigField> {
    let mut fields = Vec::new();

    if let Some(table) = value.as_table() {
        for (key, val) in table {
            let full_key = format!("{}.{}", parent_key, key);
            let description = format!("Configuration for {}", key);
            let field_type = match val {
                toml::Value::String(_) => "string",
                toml::Value::Integer(_) => "integer",
                toml::Value::Float(_) => "float",
                toml::Value::Boolean(_) => "boolean",
                toml::Value::Table(_) => {
                    let nested = extract_nested_fields(&full_key, val, current);
                    fields.extend(nested);
                    continue;
                }
                _ => "string",
            }
            .to_string();

            let current_value = get_value_by_path(current, &full_key)
                .map(|v| v.to_string().trim_matches('"').to_string())
                .unwrap_or_else(|| val.to_string().trim_matches('"').to_string());

            fields.push(ConfigField {
                key: full_key,
                value: current_value,
                field_type,
                description,
            });
        }
    }

    fields
}

fn get_value_by_path<'a>(
    toml_value: Option<&'a toml::Value>,
    path: &str,
) -> Option<&'a toml::Value> {
    let parts: Vec<&str> = path.split('.').collect();
    let mut current = toml_value?;

    for part in parts {
        current = current.get(part)?;
    }

    Some(current)
}

#[derive(Deserialize)]
pub struct UpdateConfigRequest {
    config: HashMap<String, String>,
}

pub async fn update_service_config(
    State(node): State<Node>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateConfigRequest>,
) -> impl IntoResponse {
    {
        let registry = node.registry.read().await;
        if !registry.list_definitions().iter().any(|s| s.id == id) {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({"status": false, "error": "service not found"})),
            )
                .into_response();
        }
    }

    let service_root = format!("{}/services/{}", node.config.data_dir, id);
    let config_path = format!("{}/config.toml", service_root);

    let mut toml_table = toml::map::Map::new();

    for (key, value) in payload.config {
        let parts: Vec<&str> = key.split('.').collect();
        if parts.len() == 1 {
            if let Ok(parsed_value) = toml::from_str(&value) {
                toml_table.insert(key, parsed_value);
            } else {
                toml_table.insert(key, toml::Value::String(value));
            }
        } else {
            insert_nested_value(&mut toml_table, &parts, &value);
        }
    }

    let toml_value = toml::Value::Table(toml_table);
    let toml_string = match toml::to_string_pretty(&toml_value) {
        Ok(s) => s,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": false, "error": e.to_string()})),
            )
                .into_response();
        }
    };

    if let Err(e) = fs::write(&config_path, toml_string) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": false, "error": e.to_string()})),
        )
            .into_response();
    }

    (
        StatusCode::OK,
        Json(json!({
            "status": true,
            "message": "Configuration updated. Please restart the service to apply changes."
        })),
    )
        .into_response()
}

fn insert_nested_value(
    table: &mut toml::map::Map<String, toml::Value>,
    parts: &[&str],
    value: &str,
) {
    if parts.is_empty() {
        return;
    }

    if parts.len() == 1 {
        let parsed_value = if let Ok(i) = value.parse::<i64>() {
            toml::Value::Integer(i)
        } else if let Ok(f) = value.parse::<f64>() {
            toml::Value::Float(f)
        } else if let Ok(b) = value.parse::<bool>() {
            toml::Value::Boolean(b)
        } else {
            toml::Value::String(value.to_string())
        };

        table.insert(parts[0].to_string(), parsed_value);
    } else {
        let section = parts[0];
        let remaining = &parts[1..];

        let nested_table = table
            .entry(section.to_string())
            .or_insert_with(|| toml::Value::Table(toml::map::Map::new()))
            .as_table_mut();

        if let Some(nested) = nested_table {
            insert_nested_value(nested, remaining, value);
        }
    }
}

#[derive(Deserialize)]
pub struct CreateTemplateRequest {
    fields: HashMap<String, TemplateField>,
}

#[derive(Deserialize)]
pub struct TemplateField {
    value: String,
    field_type: String,
}

pub async fn create_or_update_template(
    State(node): State<Node>,
    Path(id): Path<String>,
    Json(payload): Json<CreateTemplateRequest>,
) -> impl IntoResponse {
    {
        let registry = node.registry.read().await;
        if !registry.list_definitions().iter().any(|s| s.id == id) {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({"status": false, "error": "service not found"})),
            )
                .into_response();
        }
    }

    let service_root = format!("{}/services/{}", node.config.data_dir, id);
    let template_path = format!("{}/config.example.toml", service_root);

    let mut toml_table = toml::map::Map::new();
    for (key, field) in payload.fields {
        let value = match field.field_type.as_str() {
            "integer" => {
                if let Ok(i) = field.value.parse::<i64>() {
                    toml::Value::Integer(i)
                } else {
                    toml::Value::String(field.value)
                }
            }
            "float" => {
                if let Ok(f) = field.value.parse::<f64>() {
                    toml::Value::Float(f)
                } else {
                    toml::Value::String(field.value)
                }
            }
            "boolean" => {
                if let Ok(b) = field.value.parse::<bool>() {
                    toml::Value::Boolean(b)
                } else {
                    toml::Value::String(field.value)
                }
            }
            _ => toml::Value::String(field.value),
        };
        toml_table.insert(key, value);
    }

    let toml_value = toml::Value::Table(toml_table);
    let toml_string = match toml::to_string_pretty(&toml_value) {
        Ok(s) => s,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": false, "error": e.to_string()})),
            )
                .into_response();
        }
    };

    if let Err(e) = fs::write(&template_path, toml_string) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": false, "error": e.to_string()})),
        )
            .into_response();
    }

    (
        StatusCode::OK,
        Json(json!({
            "status": true,
            "message": "Config template created successfully"
        })),
    )
        .into_response()
}

pub async fn delete_config_template(
    State(node): State<Node>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    {
        let registry = node.registry.read().await;
        if !registry.list_definitions().iter().any(|s| s.id == id) {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({"status": false, "error": "service not found"})),
            )
                .into_response();
        }
    }

    let service_root = format!("{}/services/{}", node.config.data_dir, id);
    let template_path = format!("{}/config.example.toml", service_root);

    // Check if template exists
    if !std::path::Path::new(&template_path).exists() {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({"status": false, "error": "template not found"})),
        )
            .into_response();
    }

    // Delete the template file
    if let Err(e) = fs::remove_file(&template_path) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": false, "error": e.to_string()})),
        )
            .into_response();
    }

    (
        StatusCode::OK,
        Json(json!({
            "status": true,
            "message": "Config template deleted successfully"
        })),
    )
        .into_response()
}

pub async fn get_port_allocations(State(node): State<Node>) -> impl IntoResponse {
    let port_manager = node.port_manager.read().await;
    let allocations = port_manager.all_allocations();

    Json(json!({
        "allocations": allocations
    }))
}

pub async fn get_logs(State(node): State<Node>, Path(id): Path<String>) -> impl IntoResponse {
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

    let manager = node.manager.read().await;

    if let Some(service) = manager.list().await.iter().find(|s| s.id == id) {
        let logs = service.log_buffer.get_all().await;
        Json(json!({
            "service": id,
            "logs": logs
        }))
        .into_response()
    } else {
        Json(json!({
            "service": id,
            "logs": []
        }))
        .into_response()
    }
}

pub async fn stream_logs(
    State(node): State<Node>,
    Path(id): Path<String>,
) -> Result<
    Sse<impl futures::Stream<Item = Result<axum::response::sse::Event, Infallible>>>,
    (StatusCode, Json<serde_json::Value>),
> {
    {
        let registry = node.registry.read().await;
        if !registry.list_definitions().iter().any(|s| s.id == id) {
            return Err((
                StatusCode::NOT_FOUND,
                Json(json!({
                    "status": false,
                    "error": "service not found"
                })),
            ));
        }
    }

    let service_opt = {
        let manager = node.manager.read().await;
        let services = manager.list_cloned().await;
        services.into_iter().find(|s| s.id == id)
    };

    let stream = async_stream::stream! {
        if let Some(service) = service_opt {
            let mut last_count = 0;
            loop {
                let logs = service.log_buffer.get_recent().await;

                for log in logs.iter().skip(last_count) {
                    let data = serde_json::to_string(&log).unwrap_or_default();
                    yield Ok::<_, Infallible>(axum::response::sse::Event::default().data(data));
                }

                last_count = logs.len();
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        } else {
            loop {
                tokio::time::sleep(Duration::from_millis(1000)).await;
            }
        }
    };

    Ok(Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(10))
            .text("keep-alive"),
    ))
}

pub async fn clear_logs(State(node): State<Node>, Path(id): Path<String>) -> impl IntoResponse {
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

    let manager = node.manager.read().await;

    if let Some(service) = manager.list().await.iter().find(|s| s.id == id) {
        match service.log_buffer.clear().await {
            Ok(_) => Json(json!({
                "status": true,
                "message": "Logs cleared"
            }))
            .into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": false,
                    "error": format!("Failed to clear logs: {}", e)
                })),
            )
                .into_response(),
        }
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(json!({
                "status": false,
                "error": "service not running"
            })),
        )
            .into_response()
    }
}

#[derive(Serialize)]
pub struct ServiceStats {
    pub service_id: String,
    pub cpu_usage: f32,
    pub memory_mb: f64,
    pub pid: Option<u32>,
}

pub async fn get_service_stats(
    State(node): State<Node>,
    Path(id): Path<String>,
) -> impl IntoResponse {
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

    let (service_id, pid) = {
        let manager = node.manager.read().await;
        let services = manager.list_cloned().await;
        match services.into_iter().find(|s| s.id == id) {
            Some(s) => {
                let pid = s.get_pid().await;
                (s.id, pid)
            }
            None => {
                return Json(ServiceStats {
                    service_id: id,
                    cpu_usage: 0.0,
                    memory_mb: 0.0,
                    pid: None,
                })
                .into_response();
            }
        }
    };

    if let Some(pid) = pid {
        let mut sys = System::new_all();
        sys.refresh_all();

        if let Some(process) = sys.process(sysinfo::Pid::from_u32(pid)) {
            let stats = ServiceStats {
                service_id: service_id.clone(),
                cpu_usage: process.cpu_usage(),
                memory_mb: process.memory() as f64 / 1024.0 / 1024.0,
                pid: Some(pid),
            };

            return Json(stats).into_response();
        }
    }

    Json(ServiceStats {
        service_id,
        cpu_usage: 0.0,
        memory_mb: 0.0,
        pid,
    })
    .into_response()
}
