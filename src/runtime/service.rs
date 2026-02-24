use serde::Serialize;
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

use super::log_buffer::LogBuffer;

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

    pub working_dir: String,

    pub state: Arc<RwLock<ServiceState>>,
    pub log_buffer: LogBuffer,
    pub pid: Arc<RwLock<Option<u32>>>,
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
        working_dir: String,
    ) -> Self {
        let log_file = PathBuf::from(&working_dir).join("logs").join("service.log");
        Self {
            id,
            name,
            binary_path,
            args,
            env,
            auto_restart,
            restart_limit,
            working_dir,
            state: Arc::new(RwLock::new(ServiceState::Stopped)),
            log_buffer: LogBuffer::new(log_file),
            pid: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn set_state(&self, new_state: ServiceState) {
        let mut state = self.state.write().await;
        *state = new_state;
    }

    pub async fn get_state(&self) -> ServiceState {
        self.state.read().await.clone()
    }

    pub async fn get_pid(&self) -> Option<u32> {
        *self.pid.read().await
    }

    pub async fn set_pid(&self, new_pid: Option<u32>) {
        let mut pid = self.pid.write().await;
        *pid = new_pid;
    }
}
