mod jobs;

use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // TODO: Load environment variables
    // TODO: Initialize database connection pool
    // TODO: Setup job queue

    tracing::info!("Workers service starting");

    // TODO: Start worker loop
    // jobs::run_worker_loop().await?;

    Ok(())
}
