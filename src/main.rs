use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::node::Node;

pub mod api;
pub mod config;
pub mod identity;
pub mod node;
pub mod registry;
pub mod service;
pub mod supervisor;
pub mod supervisor_manager;

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

    {
        let mut manager = node.manager.write().await;

        for service in node.services.read().await.iter() {
            manager.start(&service).await?;
        }
    }

    api::start_api(&node).await?;
    info!("dockless shutting down");

    node.manager.write().await.shutdown_all().await;

    Ok(())
}
