use std::collections::HashMap;
use tokio::sync::broadcast;
use tokio::task::JoinHandle;

use crate::runtime::service::Service;
use crate::runtime::supervisor::Supervisor;

pub struct SupervisorHandle {
    pub shutdown_tx: broadcast::Sender<()>,
    pub join_handle: JoinHandle<()>,
}

pub struct SupervisorManager {
    services: HashMap<String, Service>,
    supervisors: HashMap<String, SupervisorHandle>,
    shutdown_tx: broadcast::Sender<()>,
    port_manager: Option<crate::platform::port_manager::PortManager>,
}

impl SupervisorManager {
    pub fn new() -> Self {
        let (shutdown_tx, _) = broadcast::channel(1);
        Self {
            services: HashMap::new(),
            supervisors: HashMap::new(),
            shutdown_tx,
            port_manager: None,
        }
    }

    pub fn register_service(&mut self, service: Service) -> anyhow::Result<()> {
        if self.services.contains_key(&service.id) {
            anyhow::bail!("service {} already registered", service.id);
        }

        self.services.insert(service.id.clone(), service);
        Ok(())
    }

    pub fn unregister_service(&mut self, id: &str) -> anyhow::Result<()> {
        if self.services.remove(id).is_none() {
            anyhow::bail!("service {} not registered", id);
        }
        if let Some(pm) = self.port_manager.as_mut() {
            let _ = pm.deallocate(id);
        }
        Ok(())
    }

    pub fn update_service(&mut self, service: Service) -> anyhow::Result<()> {
        if !self.services.contains_key(&service.id) {
            anyhow::bail!("service {} not registered", service.id);
        }
        self.services.insert(service.id.clone(), service);
        Ok(())
    }

    pub fn service_count(&self) -> usize {
        self.services.len()
    }

    pub fn running_count(&self) -> usize {
        self.supervisors.len()
    }

    pub async fn start(&mut self, id: &str) -> anyhow::Result<()> {
        if self.supervisors.contains_key(id) {
            anyhow::bail!("service {} already running", id);
        }

        let service = self
            .services
            .get(id)
            .ok_or_else(|| anyhow::anyhow!("service {} not found", id))?
            .clone();

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

    pub async fn stop(&mut self, id: &str) -> anyhow::Result<()> {
        if let Some(handle) = self.supervisors.remove(id) {
            let _ = handle.shutdown_tx.send(());

            // Try to kill the process if running
            if let Some(service) = self.services.get(id) {
                let pid = service.pid.read().await.clone();
                if let Some(pid) = pid {
                    #[cfg(unix)]
                    unsafe {
                        libc::kill(pid as i32, libc::SIGTERM);
                    }
                    // Wait a bit for graceful exit
                    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                    // If still running, force kill
                    #[cfg(unix)]
                    unsafe {
                        libc::kill(pid as i32, libc::SIGKILL);
                    }
                }
            }

            match tokio::time::timeout(tokio::time::Duration::from_secs(30), handle.join_handle)
                .await
            {
                Ok(_) => {
                    // Deallocate port
                    if let Some(pm) = self.port_manager.as_mut() {
                        let _ = pm.deallocate(id);
                    }
                    Ok(())
                }
                Err(_) => {
                    tracing::warn!("[{}] supervisor failed to stop within timeout", id);
                    // Deallocate port anyway
                    if let Some(pm) = self.port_manager.as_mut() {
                        let _ = pm.deallocate(id);
                    }
                    Ok(())
                }
            }
        } else {
            anyhow::bail!("service {} not running", id);
        }
    }

    pub async fn restart(&mut self, id: &str) -> anyhow::Result<()> {
        if self.supervisors.contains_key(id) {
            self.stop(id).await?;
        }
        self.start(&id).await
    }

    pub async fn list_ids(&self) -> Vec<String> {
        let ids: Vec<String> = self.services.keys().cloned().collect();
        ids
    }

    pub async fn list(&self) -> Vec<&Service> {
        let mut result = Vec::new();

        for (_, service) in &self.services {
            result.push(service);
        }

        result
    }

    pub async fn list_cloned(&self) -> Vec<Service> {
        self.services.values().cloned().collect()
    }

    pub async fn start_all(&mut self) -> anyhow::Result<()> {
        let ids: Vec<String> = self.services.keys().cloned().collect();

        for id in ids {
            if let Err(e) = self.start(&id).await {
                tracing::error!(error = ?e, "failed to start service {}", id);
            }
        }

        Ok(())
    }

    pub async fn shutdown_all(&mut self) {
        let _ = self.shutdown_tx.send(());

        let handles: Vec<_> = self.supervisors.drain().collect();

        for (id, handle) in handles {
            match tokio::time::timeout(tokio::time::Duration::from_secs(30), handle.join_handle)
                .await
            {
                Ok(_) => tracing::info!("[{}] supervisor stopped", id),
                Err(_) => {
                    tracing::warn!("[{}] supervisor failed to stop within timeout, forcing", id)
                }
            }
        }
    }
}
