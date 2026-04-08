mod routes;
mod handlers;
mod db;

use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // TODO: Load environment variables
    // TODO: Initialize database connection pool
    
    // Build router
    let app = Router::new()
        .route("/health", get(handlers::health))
        .route("/tx/:hash", get(handlers::get_transaction))
        .route("/address/:id", get(handlers::get_address));

    // TODO: Add middleware (CORS, logging, etc.)

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("API server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
