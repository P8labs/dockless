use std::collections::HashMap;
use tokio::sync::broadcast;
use tokio::task::JoinHandle;

use crate::service::Service;
use crate::supervisor::Supervisor;

pub struct SupervisorHandle {
    pub shutdown_tx: broadcast::Sender<()>,
    pub join_handle: JoinHandle<()>,
}

pub struct SupervisorManager {
    supervisors: HashMap<String, SupervisorHandle>,
    shutdown_tx: broadcast::Sender<()>,
}

impl SupervisorManager {
    pub fn new() -> Self {
        let (shutdown_tx, _) = broadcast::channel(1);
        Self {
            supervisors: HashMap::new(),
            shutdown_tx,
        }
    }

    pub async fn start(&mut self, service: &Service) -> anyhow::Result<()> {
        if self.supervisors.contains_key(&service.id) {
            anyhow::bail!("service {} already running", service.id);
        }

        let (service_shutdown_tx, _) = broadcast::channel(1);
        let global_shutdown_rx = self.shutdown_tx.subscribe();
        let service_shutdown_rx = service_shutdown_tx.subscribe();

        let service_clone = service.clone();

        let handle = tokio::spawn(async move {
            let mut supervisor = Supervisor::new();
            if let Err(e) = supervisor
                .run_supervised(service_clone, global_shutdown_rx, service_shutdown_rx)
                .await
            {
                tracing::error!(error = ?e, "supervisor crashed");
            }
        });

        self.supervisors.insert(
            service.id.clone(),
            SupervisorHandle {
                shutdown_tx: service_shutdown_tx,
                join_handle: handle,
            },
        );

        Ok(())
    }

    pub async fn stop(&mut self, service_id: &str) -> anyhow::Result<()> {
        if let Some(handle) = self.supervisors.remove(service_id) {
            let _ = handle.shutdown_tx.send(());
            let _ = handle.join_handle.await;
            Ok(())
        } else {
            anyhow::bail!("service {} not running", service_id);
        }
    }

    pub async fn restart(&mut self, service: &Service) -> anyhow::Result<()> {
        self.stop(&service.id).await?;
        self.start(&service).await
    }

    pub async fn shutdown_all(&mut self) {
        let _ = self.shutdown_tx.send(());

        for (_, handle) in self.supervisors.drain() {
            let _ = handle.join_handle.await;
        }
    }
}
