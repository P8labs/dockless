use serde::Serialize;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize)]
pub enum ServiceState {
    Starting,
    Running,
    Stopping,
    Stopped,
    Crashed,
}

#[derive(Clone)]
pub struct Service {
    pub id: String,
    pub name: String,
    pub binary_path: String,
    pub state: Arc<RwLock<ServiceState>>,
}

impl Service {
    pub fn new(id: String, name: String, binary_path: String) -> Self {
        Self {
            id,
            name,
            binary_path,
            state: Arc::new(RwLock::new(ServiceState::Stopped)),
        }
    }

    pub async fn set_state(&self, new_state: ServiceState) {
        let mut state = self.state.write().await;
        *state = new_state;
    }

    pub async fn get_state(&self) -> ServiceState {
        let state = self.state.read().await;
        state.clone()
    }
}
