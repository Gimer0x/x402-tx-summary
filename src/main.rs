use tokio::net::TcpListener;
use tracing_subscriber::FmtSubscriber;

mod app;
mod config;
mod controllers;
mod middleware;
mod models;
mod routes;
mod semantics;
mod services;
mod utils;
mod errors;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenvy::dotenv().ok();

    // Logging
    let subscriber = FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)?;

    let config = config::Config::from_env()?;

    let app = app::build_app(config.clone()).await?;

    let listener = TcpListener::bind(&config.server_addr).await?;
    tracing::info!("Server running on {}", config.server_addr);

    axum::serve(listener, app).await?;

    Ok(())
}
