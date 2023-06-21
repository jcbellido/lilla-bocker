use std::env;
use std::net::SocketAddr;

use axum::Router;

use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let mut target_dir_path: String = String::from("../sample_assets_00");

    let env_var_name = "MOCK_SERVE_DIR";
    tracing::info!("Checking for {env_var_name} env");
    if env::var(env_var_name).is_ok() {
        target_dir_path = env::var(env_var_name).unwrap();
    } else {
        tracing::info!("{env_var_name} environment var not found.");
    }

    if !std::path::PathBuf::from(&target_dir_path).exists() {
        panic!("Target directory: {target_dir_path} not found!");
    }

    tracing::info!("Serving from {target_dir_path}");

    let addr = SocketAddr::from(([0, 0, 0, 0], 8888));
    tracing::info!("listening on {addr}");

    let r = Router::new()
        .nest_service("/", ServeDir::new(target_dir_path))
        .into_make_service();

    axum::Server::bind(&addr).serve(r).await.unwrap();
}
