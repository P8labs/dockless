use crate::platform::node::Node;
use crate::platform::port_manager::PortManager;
use axum::{Json, Router, extract::State, response::IntoResponse, routing::get};
use serde::Serialize;

pub fn routes() -> Router<Node> {
    Router::new().route("/registry", get(get_registry))
}

#[derive(Serialize)]
struct RegistryEntry {
    id: String,
    name: String,
    ready: bool,
    binary_path: String,
    args: Vec<String>,
    env: std::collections::HashMap<String, String>,
    auto_restart: bool,
    restart_limit: Option<u32>,
    current_version: Option<String>,
    port: Option<u16>,
}

async fn get_registry(State(node): State<Node>) -> impl IntoResponse {
    let registry = node.registry.read().await;
    let manager = node.manager.read().await;
    let port_manager = node.port_manager.read().await;

    let entries: Vec<RegistryEntry> = registry
        .list_definitions()
        .iter()
        .map(|def| {
            // Try to resolve the actual listening port from the running process.
            let port = 'port: {
                // Look up the PID of the running service synchronously via a blocking read.
                // We use try_read so we don't deadlock; fall back to None on contention.
                if let Some(service) = manager.get_service(&def.id) {
                    if let Ok(pid_guard) = service.pid.try_read() {
                        if let Some(pid) = *pid_guard {
                            let listening = PortManager::get_listening_ports_for_pid(pid);
                            if !listening.is_empty() {
                                // Prefer the allocated port if the service is listening on it,
                                // otherwise report the first port the process actually bound.
                                let allocated = port_manager.get_port(&def.id);
                                if let Some(a) = allocated {
                                    if listening.contains(&a) {
                                        break 'port Some(a);
                                    }
                                }
                                break 'port Some(listening[0]);
                            }
                        }
                    }
                }
                // Service not running or not yet listening — don't expose the allocated port.
                None
            };

            RegistryEntry {
                id: def.id.clone(),
                name: def.name.clone(),
                ready: def.ready,
                binary_path: def.binary_path.clone(),
                args: def.args.clone(),
                env: def.env.clone(),
                auto_restart: def.auto_restart,
                restart_limit: def.restart_limit,
                current_version: def.current_version.clone(),
                port,
            }
        })
        .collect();

    Json(entries)
}
