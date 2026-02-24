use crate::{
    config::{Config, load_config},
    identity,
    platform::port_manager::PortManager,
    registry::RegistryManager,
    runtime::{service::Service, supervisor_manager::SupervisorManager},
};
use anyhow::{Context, Result};
use std::{fs, sync::Arc};
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct Node {
    pub node_id: String,
    pub config: Config,
    pub registry: Arc<RwLock<RegistryManager>>,
    pub manager: Arc<RwLock<SupervisorManager>>,
    pub port_manager: Arc<RwLock<PortManager>>,
}

impl Node {
    pub fn new() -> Result<Self> {
        let config = load_config()?;
        let node_id = identity::load_or_create_identity(&config.node_id)?;

        fs::create_dir_all(&config.data_dir).context("failed to create data directory")?;
        let registry_path = format!("{}/projects.json", config.data_dir);
        let registry = RegistryManager::load_or_init(&registry_path)?;

        let ports_path = format!("{}/ports.json", config.data_dir);
        let mut port_manager = PortManager::load_or_init(&ports_path)?;

        let definitions = registry.list_definitions();

        let mut manager = SupervisorManager::new();

        for def in definitions {
            // Skip services that are not ready
            if !def.ready {
                continue;
            }

            let service_root = format!("{}/services/{}", config.data_dir, def.id);
            let bin_dir = format!("{}/bin", service_root);
            let data_dir = format!("{}/data", service_root);
            let logs_dir = format!("{}/logs", service_root);

            fs::create_dir_all(&bin_dir)
                .with_context(|| format!("failed to create bin dir for {}", def.id))?;
            fs::create_dir_all(&data_dir)
                .with_context(|| format!("failed to create data dir for {}", def.id))?;
            fs::create_dir_all(&logs_dir)
                .with_context(|| format!("failed to create logs dir for {}", def.id))?;

            let mut env = def.env.clone();

            // Allocate port if not already allocated
            let port = port_manager.allocate(&def.id)?;
            env.insert("PORT".to_string(), port.to_string());

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

            manager.register_service(service)?;
        }

        return Ok(Self {
            node_id,
            config,
            registry: Arc::new(RwLock::new(registry)),
            manager: Arc::new(RwLock::new(manager)),
            port_manager: Arc::new(RwLock::new(port_manager)),
        });
    }
}
