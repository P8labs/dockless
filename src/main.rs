use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::node::Node;

pub mod api;
pub mod config;
pub mod identity;
pub mod node;

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
    info!("Starting dockless runtime");
    info!("Version 0.0.1");

    let node = Node::new()?;

    info!("Node Id: {}", node.node_id);

    api::start_api(node).await?;
    info!("dockless shutting down");

    Ok(())
}
