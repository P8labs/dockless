use serde::Serialize;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize)]
pub enum ServiceState {
    Starting,
    Running,
    Stopping,
    Stopped,
    Crashed,
    Failed,
}

#[derive(Clone)]
pub struct Service {
    pub id: String,
    pub name: String,
    pub binary_path: String,

    pub args: Vec<String>,
    pub env: HashMap<String, String>,
    pub auto_restart: bool,
    pub restart_limit: Option<u32>,

    pub state: Arc<RwLock<ServiceState>>,
}

impl Service {
    pub fn new(
        id: String,
        name: String,
        binary_path: String,
        args: Vec<String>,
        env: HashMap<String, String>,
        auto_restart: bool,
        restart_limit: Option<u32>,
    ) -> Self {
        Self {
            id,
            name,
            binary_path,
            args,
            env,
            auto_restart,
            restart_limit,
            state: Arc::new(RwLock::new(ServiceState::Stopped)),
        }
    }

    pub async fn set_state(&self, new_state: ServiceState) {
        let mut state = self.state.write().await;
        *state = new_state;
    }

    pub async fn get_state(&self) -> ServiceState {
        self.state.read().await.clone()
    }
}
