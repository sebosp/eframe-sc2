//! HTTP server, routes and proxy

use axum::{
    body::Body,
    extract::ws::{WebSocket, WebSocketUpgrade},
    http::{Method, Request, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    routing::Router,
};
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::upgrade::Upgraded;
use hyper_util::rt::TokioIo;
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tower::Service;
use tower::ServiceExt;
use tower_http::services::ServeDir;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

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
    let shared_state = AppState { source_dir };
    let router_svc = Router::new()
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .with_state(shared_state.clone())
        .nest(
            "/api",
            crate::api::routes(axum::extract::State(shared_state.clone())),
        )
        .route("/_trunk/ws", get(web_socket_handler))
        .nest_service("/", ServeDir::new("dist"));

    // From axum/examples/http-proxy/src/main.rs
    let tower_service = tower::service_fn(move |req: Request<_>| {
        let router_svc = router_svc.clone();
        let req = req.map(Body::new);
        async move {
            if req.method() == Method::CONNECT {
                proxy(req).await
            } else {
                router_svc.oneshot(req).await.map_err(|err| match err {})
            }
        }
    });

    let hyper_service = hyper::service::service_fn(move |request: Request<Incoming>| {
        tower_service.clone().call(request)
    });

    let addr: SocketAddr = format!("{}:{}", ip, port)
        .parse()
        .expect("Invalid IP address");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let io = TokioIo::new(stream);
        let hyper_service = hyper_service.clone();
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .preserve_header_case(true)
                .title_case_headers(true)
                .serve_connection(io, hyper_service)
                .with_upgrades()
                .await
            {
                println!("Failed to serve connection: {:?}", err);
            }
        });
    }
}

async fn web_socket_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            // client disconnected
            return;
        };

        if socket.send(msg).await.is_err() {
            // client disconnected
            return;
        }
    }
}

async fn proxy(req: Request<Body>) -> Result<Response, hyper::Error> {
    tracing::trace!(?req);

    if let Some(host_addr) = req.uri().authority().map(|auth| auth.to_string()) {
        tokio::task::spawn(async move {
            match hyper::upgrade::on(req).await {
                Ok(upgraded) => {
                    if let Err(e) = tunnel(upgraded, host_addr).await {
                        tracing::warn!("server io error: {}", e);
                    };
                }
                Err(e) => tracing::warn!("upgrade error: {}", e),
            }
        });

        Ok(Response::new(axum::body::Body::empty()))
    } else {
        tracing::warn!("CONNECT host is not socket addr: {:?}", req.uri());
        Ok((
            StatusCode::BAD_REQUEST,
            "CONNECT must be to a socket address",
        )
            .into_response())
    }
}

// From axum/examples/http-proxy/src/main.rs
async fn tunnel(upgraded: Upgraded, addr: String) -> std::io::Result<()> {
    let mut server = TcpStream::connect(addr).await?;
    let mut upgraded = TokioIo::new(upgraded);

    let (from_client, from_server) =
        tokio::io::copy_bidirectional(&mut upgraded, &mut server).await?;

    tracing::debug!(
        "client wrote {} bytes and received {} bytes",
        from_client,
        from_server
    );

    Ok(())
}
