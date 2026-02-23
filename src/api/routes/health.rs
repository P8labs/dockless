use std::path::Path;

use crate::platform::node::Node;
use axum::{Json, Router, extract::State, routing::get};
use serde::Serialize;
use sysinfo::{Disks, System};

#[derive(Serialize)]
struct SystemStats {
    cpu_usage: f32,
    memory_used: u64,
    memory_total: u64,
    disk_used: u64,
    disk_total: u64,
    uptime: u64,
}

#[derive(Serialize)]
struct Health {
    name: String,
    status: String,
    node_id: String,
    service_total: usize,
    service_running: usize,
    stats: SystemStats,
}
pub fn routes() -> Router<Node> {
    Router::new().route("/health", get(health_index))
}

async fn health_index(State(node): State<Node>) -> Json<Health> {
    let mut system = System::new_all();
    system.refresh_all();

    let cpu_usage = system.global_cpu_usage();

    let memory_used = system.used_memory();
    let memory_total = system.total_memory();

    let disks = Disks::new_with_refreshed_list();

    let mut disk_used = 0;
    let mut disk_total = 0;

    let root = disks
        .list()
        .iter()
        .find(|d| d.mount_point() == Path::new("/"));

    if let Some(disk) = root {
        disk_total = disk.total_space();
        disk_used = disk.total_space() - disk.available_space();
    }

    let uptime = System::uptime();

    let manager = node.manager.read().await;
    let service_total = manager.service_count();
    let service_running = manager.running_count();

    Json(Health {
        name: "dockless".to_string(),
        status: "alive".to_string(),
        node_id: node.node_id.clone(),
        service_total,
        service_running,
        stats: SystemStats {
            cpu_usage,
            memory_used,
            memory_total,
            disk_used,
            disk_total,
            uptime,
        },
    })
}
