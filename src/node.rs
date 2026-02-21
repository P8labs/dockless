use anyhow::{Ok, Result};

use crate::identity;

#[derive(Clone)]
pub struct Node {
    pub node_id: String,
}

impl Node {
    pub fn new() -> Result<Self> {
        let id = identity::load_or_create_identity("./node_id")?;
        return Ok(Self { node_id: id });
    }
}
