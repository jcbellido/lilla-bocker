use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::Result;
use axum::{extract::State, routing::get, Json, Router};
use clap::Parser;
use flipbook::flipbook::package::FlipbookPackage;
use serde::Serialize;
use tower_http::compression::CompressionLayer;
use tower_http::services::ServeDir;

mod args;
use args::Args;

struct AppState {
    path_flipbooks: std::path::PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    tracing::debug!("Arguments read: {:#?}", args);
    tracing::info!("Starting dev server");
    tracing::info!("Trying to serve files from: `{}`", args.serve);

    if !std::path::PathBuf::from(&args.serve).exists() {
        anyhow::bail!("Target directory: {} not found!", &args.serve);
    }

    let shared_state = Arc::new(AppState {
        path_flipbooks: std::path::PathBuf::from(&args.serve),
    });

    tracing::info!("Opening 0.0.0.0:{}", args.port);
    let addr = SocketAddr::from(([0, 0, 0, 0], args.port));

    let r = Router::new()
        .nest_service(
            "/flipbooks",
            ServeDir::new(std::path::PathBuf::from(args.serve)),
        )
        .route("/api/all-v1", get(all_v1))
        .layer(CompressionLayer::new())
        .with_state(shared_state)
        .into_make_service();

    axum::Server::bind(&addr).serve(r).await?;
    Ok(())
}

#[derive(Debug, Default, Serialize)]
struct AllFlipbooks {
    pub payload: Vec<FlipbookPackage>,
}

/// Returns all the available Flipbooks under {{AppState.path_flipbooks}} by verifying they can be
///   deserialized into proper "packaged flipbooks". In case of error it'll transform {{anyhow::Error}} into {{String}}
async fn all_v1(State(state): State<Arc<AppState>>) -> Result<Json<AllFlipbooks>, String> {
    match gather_all_metadata(state).await {
        Ok(answer) => Ok(answer),
        Err(e) => Err(e.to_string()),
    }
}

async fn gather_all_metadata(state: Arc<AppState>) -> Result<Json<AllFlipbooks>, anyhow::Error> {
    tracing::info!("Calling all_v1 with {:#?}", state.path_flipbooks);

    let mut rd = tokio::fs::read_dir(&state.path_flipbooks).await?;
    let mut payload = vec![];
    loop {
        let Some(entry) = rd.next_entry().await? else {
            break;
        };

        if let Some(f_extension) = entry.path().extension() {
            if f_extension != "json" {
                continue;
            }
        }

        let s = tokio::fs::read_to_string(entry.path()).await?;

        let flipbook: FlipbookPackage = match serde_json::from_str(&s) {
            Ok(o) => o,
            Err(e) => {
                tracing::error!(
                    "Error reading json content of `{:#?}`: {}",
                    entry.path(),
                    e.to_string()
                );
                continue;
            }
        };

        payload.push(flipbook);
    }

    Ok(Json(AllFlipbooks { payload }))
}
