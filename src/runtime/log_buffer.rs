use serde::Serialize;
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::RwLock;

const MAX_LOG_LINES: usize = 1000;

#[derive(Debug, Clone, Serialize)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: String,
    pub message: String,
}

#[derive(Clone)]
pub struct LogBuffer {
    logs: Arc<RwLock<VecDeque<LogEntry>>>,
}

impl LogBuffer {
    pub fn new() -> Self {
        Self {
            logs: Arc::new(RwLock::new(VecDeque::with_capacity(MAX_LOG_LINES))),
        }
    }

    pub async fn push(&self, level: String, message: String) {
        let mut logs = self.logs.write().await;

        if logs.len() >= MAX_LOG_LINES {
            logs.pop_front();
        }

        logs.push_back(LogEntry {
            timestamp: chrono::Utc::now().to_rfc3339(),
            level,
            message,
        });
    }

    pub async fn get_all(&self) -> Vec<LogEntry> {
        self.logs.read().await.iter().cloned().collect()
    }

    pub async fn clear(&self) {
        self.logs.write().await.clear();
    }
}
