use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use tokio::sync::RwLock;

const MAX_BUFFER_LINES: usize = 100;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: String,
    pub message: String,
}

#[derive(Clone)]
pub struct LogBuffer {
    logs: Arc<RwLock<VecDeque<LogEntry>>>,
    log_file_path: PathBuf,
}

impl LogBuffer {
    pub fn new(log_file_path: PathBuf) -> Self {
        Self {
            logs: Arc::new(RwLock::new(VecDeque::with_capacity(MAX_BUFFER_LINES))),
            log_file_path,
        }
    }

    pub async fn push(&self, level: String, message: String) {
        let entry = LogEntry {
            timestamp: chrono::Utc::now().to_rfc3339(),
            level: level.clone(),
            message: message.clone(),
        };

        {
            let mut logs = self.logs.write().await;
            if logs.len() >= MAX_BUFFER_LINES {
                logs.pop_front();
            }
            logs.push_back(entry.clone());
        }

        if let Err(e) = self.write_to_file(&entry).await {
            eprintln!("Failed to write log to file: {}", e);
        }
    }

    async fn write_to_file(&self, entry: &LogEntry) -> std::io::Result<()> {
        let json_line = serde_json::to_string(entry)?;
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_file_path)
            .await?;
        file.write_all(format!("{}\n", json_line).as_bytes())
            .await?;
        file.flush().await?;
        Ok(())
    }

    pub async fn get_all(&self) -> Vec<LogEntry> {
        match self.read_from_file().await {
            Ok(logs) => logs,
            Err(e) => {
                eprintln!("Failed to read logs from file: {}", e);
                vec![]
            }
        }
    }

    pub async fn get_recent(&self) -> Vec<LogEntry> {
        self.logs.read().await.iter().cloned().collect()
    }

    async fn read_from_file(&self) -> std::io::Result<Vec<LogEntry>> {
        use tokio::io::AsyncBufReadExt;

        let file = match tokio::fs::File::open(&self.log_file_path).await {
            Ok(f) => f,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                return Ok(vec![]);
            }
            Err(e) => return Err(e),
        };

        let reader = tokio::io::BufReader::new(file);
        let mut lines = reader.lines();
        let mut logs = Vec::new();

        while let Some(line) = lines.next_line().await? {
            if let Ok(entry) = serde_json::from_str::<LogEntry>(&line) {
                logs.push(entry);
            }
        }

        Ok(logs)
    }

    pub async fn clear(&self) -> std::io::Result<()> {
        self.logs.write().await.clear();

        tokio::fs::write(&self.log_file_path, "").await?;
        Ok(())
    }
}
