use anyhow::Result;

use crate::{
    config::{Config, load_config},
    identity,
    service::Service,
};

#[derive(Clone)]
pub struct Node {
    pub node_id: String,
    pub config: Config,
    pub services: Vec<Service>,
}

impl Node {
    pub fn new() -> Result<Self> {
        let id = identity::load_or_create_identity("./node_id")?;
        let config = load_config()?;

        // Temporary: create one hardcoded service
        let service = Service::new(
            "foxd".to_string(),
            "foxd".to_string(),
            "./demo/foxd".to_string(),
        );
        return Ok(Self {
            node_id: id,
            config,
            services: vec![service],
        });
    }
}
