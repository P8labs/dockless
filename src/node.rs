use crate::{
    config::{Config, load_config},
    identity,
    registry::RegistryManager,
    service::Service,
    supervisor_manager::SupervisorManager,
};
use anyhow::{Context, Result};
use std::{fs, sync::Arc};
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct Node {
    pub node_id: String,
    pub config: Config,
    pub services: Arc<RwLock<Vec<Service>>>,
    pub registry: Arc<RwLock<RegistryManager>>,
    pub manager: Arc<RwLock<SupervisorManager>>,
}

impl Node {
    pub fn new() -> Result<Self> {
        let config = load_config()?;
        let node_id = identity::load_or_create_identity(&config.node_id)?;

        fs::create_dir_all(&config.data_dir).context("failed to create data directory")?;
        let registry_path = format!("{}/projects.json", config.data_dir);
        let registry = RegistryManager::load_or_init(&registry_path)?;
        let services = registry.to_services()?;
        let manager = SupervisorManager::new();

        return Ok(Self {
            node_id,
            config,
            services: Arc::new(RwLock::new(services)),
            registry: Arc::new(RwLock::new(registry)),
            manager: Arc::new(RwLock::new(manager)),
        });
    }
}
