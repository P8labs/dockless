use tokio::sync::broadcast;
use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{node::Node, supervisor::Supervisor};

pub mod api;
pub mod config;
pub mod identity;
pub mod node;
pub mod service;
pub mod supervisor;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "dockless=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    if let Err(e) = run().await {
        error!(error = ?e, "dockless failed to start");
        std::process::exit(1);
    }
}

async fn run() -> anyhow::Result<()> {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    info!("Starting dockless runtime");
    info!("Version {}", VERSION);

    let node = Node::new()?;
    info!("Node Id: {}", node.node_id);

    let (shutdown_tx, _) = broadcast::channel::<()>(1);

    let mut handles = Vec::new();

    for service in node.services.clone() {
        let mut supervisor = Supervisor::new();
        let shutdown_rx = shutdown_tx.subscribe();

        let handle = tokio::spawn(async move {
            if let Err(e) = supervisor.run_supervised(service, shutdown_rx).await {
                tracing::error!(error = ?e, "supervisor crashed");
            }
        });

        handles.push(handle);
    }

    api::start_api(node).await?;
    info!("dockless shutting down");

    let _ = shutdown_tx.send(());
    for handle in handles {
        let _ = handle.await;
    }
    Ok(())
}
