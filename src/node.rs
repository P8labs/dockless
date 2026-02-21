use anyhow::{Ok, Result};

use crate::{
    config::{Config, load_config},
    identity,
};

#[derive(Clone)]
pub struct Node {
    pub node_id: String,
    pub config: Config,
}

impl Node {
    pub fn new() -> Result<Self> {
        let id = identity::load_or_create_identity("./node_id")?;
        let config = load_config()?;

        return Ok(Self {
            node_id: id,
            config,
        });
    }
}
