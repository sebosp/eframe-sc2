//! Server module
use axum::{routing::get, Router};
use std::sync::Arc;

/// Basic state to share through the routes
#[derive(Clone)]
pub struct AppState {
    /// The path to the IPC files
    pub source_dir: String,
}

pub async fn start_server(cli: &crate::cli::Cli) {
    let port = cli.port;
    let ip = cli.ip.clone();
    let source_dir = cli.source_dir.clone();
    tracing::info!("Starting server on {}:{}", ip, port);
    // Start a backend thread to serve requests
    tokio::spawn(async move {
        let shared_state = Arc::new(AppState { source_dir });
        let app = Router::new()
            .route("/", get(root))
            .route(
                "/api/v1/maps",
                get(crate::api::v1::details::route_query_maps),
            )
            .with_state(shared_state);

        axum::Server::bind(
            &format!("{}:{}", ip, port)
                .parse()
                .expect("Invalid IP address"),
        )
        .serve(app.into_make_service())
        .await
        .unwrap();
    });
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "this should serve the html version"
}
