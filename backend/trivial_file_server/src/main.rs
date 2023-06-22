use std::net::SocketAddr;

use anyhow::Result;
use axum::Router;
use clap::Parser;
use tower_http::services::ServeDir;

mod args;
use args::Args;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    tracing::debug!("Arguments read: {:#?}", args);

    tracing::info!("Trying to serve files from: `{}`", args.serve);

    if !std::path::PathBuf::from(&args.serve).exists() {
        anyhow::bail!("Target directory: {} not found!", &args.serve);
    }

    tracing::info!("Opening 0.0.0.0:{}", args.port);
    let addr = SocketAddr::from(([0, 0, 0, 0], args.port));

    let r = Router::new()
        .nest_service("/", ServeDir::new(std::path::PathBuf::from(args.serve)))
        .into_make_service();

    axum::Server::bind(&addr).serve(r).await?;
    Ok(())
}
