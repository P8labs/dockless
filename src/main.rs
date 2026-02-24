use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::platform::node::Node;

mod api;
mod config;
mod identity;
mod platform;
mod registry;
mod runtime;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "dockless=trace,tower_http=info".into()),
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
        manager.start_all().await?;
    }

    api::server::start_api(&node).await?;
    info!("dockless shutting down");

    let shutdown_future = async {
        node.manager.write().await.shutdown_all().await;
    };

    match tokio::time::timeout(tokio::time::Duration::from_secs(60), shutdown_future).await {
        Ok(_) => info!("all services stopped gracefully"),
        Err(_) => {
            error!("shutdown timed out after 60s, forcing exit");
        }
    }

    Ok(())
}
