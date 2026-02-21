use anyhow::{Context, Result};
use std::process::Stdio;
use tokio::{
    process::{Child, Command},
    sync::broadcast,
    time::{Duration, sleep},
};
use tracing::info;

use crate::service::{Service, ServiceState};

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
        mut shutdown_rx: broadcast::Receiver<()>,
    ) -> Result<()> {
        loop {
            info!("starting child process: {}", service.binary_path);
            service.set_state(ServiceState::Starting).await;

            let child = Command::new(&service.binary_path)
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
                _ = shutdown_rx.recv() => {
                    info!("shutdown received in supervisor");
                    service.set_state(ServiceState::Stopping).await;

                    if let Some(child) = &mut self.child {
                        info!("terminating child process");
                        let _ = child.kill().await;
                    }

                    service.set_state(ServiceState::Stopped).await;
                    break;
                }

                result = wait_future => {
                    let status = result?;

                    info!("child exited with status: {}", status);

                    if status.success() {
                        service.set_state(ServiceState::Stopped).await;
                    } else {
                        service.set_state(ServiceState::Crashed).await;
                    }

                    self.child = None;

                    info!("restarting child in 3 seconds...");
                    sleep(Duration::from_secs(3)).await;
                }
            }
        }

        info!("supervisor exiting cleanly");
        Ok(())
    }
}
