use anyhow::{Context, Result};
use std::process::Stdio;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::{Child, Command},
    sync::broadcast,
    time::{Duration, sleep, timeout},
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

    async fn graceful_stop(child: &mut Child) {
        #[cfg(unix)]
        {
            if let Some(pid) = child.id() {
                info!("Sending SIGTERM to PID {}", pid);
                unsafe {
                    libc::kill(pid as i32, libc::SIGTERM);
                }

                let wait_result = timeout(Duration::from_secs(5), child.wait()).await;

                match wait_result {
                    Ok(Ok(status)) => {
                        info!("Process {} exited gracefully with status: {}", pid, status);
                        return;
                    }
                    Ok(Err(e)) => {
                        info!("Error waiting for process {}: {}", pid, e);
                    }
                    Err(_) => {
                        info!(
                            "Process {} did not exit after SIGTERM, sending SIGKILL",
                            pid
                        );
                    }
                }
            }
        }

        let _ = child.kill().await;
        let _ = child.wait().await;
    }

    pub async fn run_supervised(
        &mut self,
        service: Service,
        mut global_shutdown_rx: broadcast::Receiver<()>,
        mut service_shutdown_rx: broadcast::Receiver<()>,
    ) -> Result<()> {
        let mut restart_count = 0;
        loop {
            info!(
                "starting child process: {} from working_dir: {}",
                service.binary_path, service.working_dir
            );
            service.set_state(ServiceState::Starting).await;

            let full_binary_path =
                std::path::Path::new(&service.working_dir).join(&service.binary_path);
            if !full_binary_path.exists() {
                let err_msg = format!(
                    "Binary not found at: {} (resolved to: {})",
                    service.binary_path,
                    full_binary_path.display()
                );
                tracing::error!("{}", err_msg);
                service.set_state(ServiceState::Failed).await;
                service
                    .log_buffer
                    .push("error".to_string(), err_msg.clone())
                    .await;
                anyhow::bail!("{}", err_msg);
            }

            let mut cmd = Command::new(&service.binary_path);
            cmd.args(&service.args);
            cmd.current_dir(&service.working_dir);
            for (k, v) in &service.env {
                cmd.env(k, v);
            }

            cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

            let mut child = match cmd.spawn() {
                Ok(c) => c,
                Err(e) => {
                    tracing::error!("failed to spawn child process: {}", e);
                    service.set_state(ServiceState::Failed).await;
                    service
                        .log_buffer
                        .push("error".to_string(), format!("Failed to start: {}", e))
                        .await;
                    anyhow::bail!(
                        "failed to spawn child process at {}: {}",
                        service.binary_path,
                        e
                    );
                }
            };

            if let Some(pid) = child.id() {
                service.set_pid(Some(pid)).await;
            }

            if let Some(stdout) = child.stdout.take() {
                let log_buffer = service.log_buffer.clone();
                let service_id = service.id.clone();
                tokio::spawn(async move {
                    let reader = BufReader::new(stdout);
                    let mut lines = reader.lines();
                    while let Ok(Some(line)) = lines.next_line().await {
                        log_buffer.push("info".to_string(), line).await;
                    }
                    info!("[{}] stdout reader finished", service_id);
                });
            }

            if let Some(stderr) = child.stderr.take() {
                let log_buffer = service.log_buffer.clone();
                let service_id = service.id.clone();
                tokio::spawn(async move {
                    let reader = BufReader::new(stderr);
                    let mut lines = reader.lines();
                    while let Ok(Some(line)) = lines.next_line().await {
                        log_buffer.push("error".to_string(), line).await;
                    }
                    info!("[{}] stderr reader finished", service_id);
                });
            }

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
                        Self::graceful_stop(child).await;
                    }

                    service.set_pid(None).await;
                    service.set_state(ServiceState::Stopped).await;
                    break;
                }

                _ = service_shutdown_rx.recv() => {
                    info!("[{}] service shutdown received in supervisor", service.id);
                    service.set_state(ServiceState::Stopping).await;

                    if let Some(child) = &mut self.child {
                        Self::graceful_stop(child).await;
                    }

                    service.set_pid(None).await;
                    service.set_state(ServiceState::Stopped).await;
                    break;
                }

                result = wait_future => {
                    let status = result?;

                    info!("[{}] child exited with status: {}", service.id, status);

                    self.child = None;
                    service.set_pid(None).await;

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
