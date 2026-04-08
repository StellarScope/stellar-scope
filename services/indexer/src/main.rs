mod ingestion;
mod decoding;
mod pipeline;

use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // TODO: Load environment variables
    // TODO: Initialize database connection pool
    // TODO: Connect to blockchain data source

    tracing::info!("Indexer service starting");

    // TODO: Start ingestion pipeline
    // pipeline::run().await?;

    Ok(())
}
