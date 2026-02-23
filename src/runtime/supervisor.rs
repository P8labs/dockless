use anyhow::{Context, Result};
use std::process::Stdio;
use tokio::{
    process::{Child, Command},
    sync::broadcast,
    time::{Duration, sleep},
};
use tracing::info;

use crate::runtime::service::{Service, ServiceState};

pub struct Supervisor {
    child: Option<Child>,
}

impl Supervisor {
    pub fn new() -> Self {
        Self { child: None }
    }

    pub async fn run_supervised(
        &mut self,
        service: Service,
        mut global_shutdown_rx: broadcast::Receiver<()>,
        mut service_shutdown_rx: broadcast::Receiver<()>,
    ) -> Result<()> {
        let mut restart_count = 0;
        loop {
            info!("starting child process: {}", service.binary_path);
            service.set_state(ServiceState::Starting).await;

            let mut cmd = Command::new(&service.binary_path);
            cmd.args(&service.args);
            cmd.current_dir(&service.working_dir);
            for (k, v) in &service.env {
                cmd.env(k, v);
            }

            let child = cmd
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()
                .with_context(|| {
                    format!("failed to spawn child process at {}", service.binary_path)
                })?;
            self.child = Some(child);

            service.set_state(ServiceState::Running).await;

            let wait_future = async {
                if let Some(child) = &mut self.child {
                    child
                        .wait()
                        .await
                        .context("failed while waiting for child process")
                } else {
                    unreachable!()
                }
            };

            tokio::select! {
                _ = global_shutdown_rx.recv() => {
                    info!("[{}] global shutdown received in supervisor", service.id);
                    service.set_state(ServiceState::Stopping).await;

                    if let Some(child) = &mut self.child {
                        let _ = child.kill().await;
                        let _ = child.wait().await;
                    }

                    service.set_state(ServiceState::Stopped).await;
                    break;
                }

                _ = service_shutdown_rx.recv() => {
                    info!("[{}] service shutdown received in supervisor", service.id);
                    service.set_state(ServiceState::Stopping).await;

                    if let Some(child) = &mut self.child {
                        let _ = child.kill().await;
                        let _ = child.wait().await;
                    }

                    service.set_state(ServiceState::Stopped).await;
                    break;
                }

                result = wait_future => {
                    let status = result?;

                    info!("[{}] child exited with status: {}", service.id, status);

                    self.child = None;

                    if global_shutdown_rx.try_recv().is_ok() ||
                       service_shutdown_rx.try_recv().is_ok() {
                        break;
                    }
                    if status.success() {
                        restart_count = 0;
                        service.set_state(ServiceState::Stopped).await;
                    } else {
                        service.set_state(ServiceState::Crashed).await;
                        restart_count += 1;
                    }

                    if !service.auto_restart {
                        break;
                    }


                    if let Some(limit) = service.restart_limit {
                        if restart_count >= limit {
                            service.set_state(ServiceState::Failed).await;
                            break;
                        }
                    }

                    info!("[{}] restarting child in 3 seconds...", service.id);
                    sleep(Duration::from_secs(3)).await;
                }
            }
        }

        info!("supervisor exiting cleanly");
        Ok(())
    }
}
