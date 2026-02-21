use crate::node::Node;

pub mod api;
pub mod identity;
pub mod node;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let node = Node::new()?;
    let _ = api::start_api(node).await;
    Ok(())
}
