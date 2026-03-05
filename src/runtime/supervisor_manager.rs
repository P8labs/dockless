use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, broadcast};
use tokio::task::JoinHandle;

use crate::platform::port_manager::PortManager;
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
    port_manager: Option<Arc<RwLock<PortManager>>>,
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

    pub fn set_port_manager(&mut self, pm: Arc<RwLock<PortManager>>) {
        self.port_manager = Some(pm);
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

    pub fn get_service(&self, id: &str) -> Option<&Service> {
        self.services.get(id)
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

        // Spawn a one-shot port verification task: after a grace period, read the
        // process's actual listening ports from /proc and check for conflicts with
        // ports already allocated to other services.  On conflict, the service is
        // stopped and the error is written to the log buffer.
        if let Some(pm) = &self.port_manager {
            let service_monitor = service.clone();
            let pm_clone = Arc::clone(pm);
            let shutdown_tx_clone = service_shutdown_tx.clone();

            tokio::spawn(async move {
                // Give the process time to bind its listening socket.
                tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

                let pid = match service_monitor.get_pid().await {
                    Some(p) => p,
                    None => {
                        tracing::warn!(
                            "[{}] no PID found after grace period, skipping port check",
                            service_monitor.id
                        );
                        return;
                    }
                };

                let listening_ports = PortManager::get_listening_ports_for_pid(pid);

                if listening_ports.is_empty() {
                    tracing::warn!(
                        "[{}] service (pid {}) is not listening on any TCP ports",
                        service_monitor.id,
                        pid
                    );
                    return;
                }

                tracing::info!(
                    "[{}] service (pid {}) listening on ports: {:?}",
                    service_monitor.id,
                    pid,
                    listening_ports
                );

                let pm = pm_clone.read().await;
                if let Some((conflict_port, conflict_service)) =
                    pm.find_conflict(&service_monitor.id, &listening_ports)
                {
                    let msg = format!(
                        "port conflict: port {} is already allocated to service '{}'",
                        conflict_port, conflict_service
                    );
                    tracing::error!("[{}] {}", service_monitor.id, msg);
                    service_monitor
                        .log_buffer
                        .push("error".to_string(), msg)
                        .await;
                    service_monitor
                        .set_state(crate::runtime::service::ServiceState::Failed)
                        .await;
                    let _ = shutdown_tx_clone.send(());
                }
            });
        }

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
                Ok(_) => Ok(()),
                Err(_) => {
                    tracing::warn!("[{}] supervisor failed to stop within timeout", id);
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
