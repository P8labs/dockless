use crate::platform::node::Node;
use axum::extract::{Multipart, Path, State};
use axum::{
    Json, Router,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
};
use serde_json::json;
use std::fs;

use serde::{Deserialize, Serialize};

use crate::{
    registry::ServiceDefinition,
    runtime::service::{Service, ServiceState},
};

pub fn routes() -> Router<Node> {
    Router::new()
        .route("/services", get(list_services))
        .route("/services", post(create_service))
        .route("/services/{id}", delete(delete_service))
        .route("/services/{id}/start", post(start_service))
        .route("/services/{id}/stop", post(stop_service))
        .route("/services/{id}/restart", post(restart_service))
        .route("/services/{id}/artifact/upload", post(upload_artifact))
        .route(
            "/services/{id}/artifact/github",
            post(install_github_artifact),
        )
        .route("/services/{id}/artifact", get(get_artifact_info))
}

#[derive(Serialize)]
struct ServiceInfo {
    id: String,
    name: String,
    state: ServiceState,
}
async fn list_services(State(node): State<Node>) -> impl IntoResponse {
    let mut services = Vec::new();

    for service in node.manager.read().await.list().await.iter() {
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
        service_root.clone(),
    );

    {
        let mut manager = node.manager.write().await;
        let _ = manager.register_service(service.clone());

        if let Err(e) = manager.start(&service.id).await {
            let mut registry = node.registry.write().await;
            let _ = registry.remove(&def.id);
            let _ = registry.save();

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

    // Update symlink: bin/current -> versions/<version>
    let current_link = format!("{}/current", bin_dir);
    let _ = fs::remove_file(&current_link);

    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        if let Err(e) = symlink(&version_dir, &current_link) {
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
        }

        let _ = registry.save();
    }

    {
        let mut manager = node.manager.write().await;
        let _ = manager.restart(&id).await;
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

    let current_link = format!("{}/current", bin_dir);
    let _ = std::fs::remove_file(&current_link);

    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        if let Err(e) = symlink(&version_dir, &current_link) {
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
        }

        let _ = registry.save();
    }

    {
        let mut manager = node.manager.write().await;
        let _ = manager.restart(&id).await;
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
