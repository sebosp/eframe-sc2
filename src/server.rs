//! HTTP server, routes and proxy

use axum::{
    body::{self, boxed, Body, BoxBody},
    extract::ws::{WebSocket, WebSocketUpgrade},
    http::{Method, Request, StatusCode, Uri},
    response::{IntoResponse, Response},
    routing::get,
    routing::Router,
};
use hyper::upgrade::Upgraded;
use tokio::net::TcpStream;
use tower::{make::Shared, ServiceExt};
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
    tokio::spawn(async move {
        let shared_state = AppState { source_dir };
        let router_svc = Router::new()
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                    .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
            )
            .nest(
                "/api",
                crate::api::routes(axum::extract::State(shared_state.clone())),
            )
            .route("/_trunk/ws", get(web_socket_handler))
            .nest_service("/", get(static_dir_handler));

        let proxy_service = tower::service_fn(move |req: Request<Body>| {
            let router_svc = router_svc.clone();
            async move {
                if req.method() == Method::CONNECT {
                    proxy(req).await
                } else {
                    router_svc.oneshot(req).await.map_err(|err| match err {})
                }
            }
        });

        axum::Server::bind(
            &format!("{}:{}", ip, port)
                .parse()
                .expect("Invalid IP address"),
        )
        .http1_preserve_header_case(true)
        .http1_title_case_headers(true)
        .serve(Shared::new(proxy_service))
        .await
        .unwrap();
    });
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

async fn static_dir_handler(uri: Uri) -> Result<Response<BoxBody>, (StatusCode, String)> {
    let res = get_static_file(uri.clone()).await?;

    if res.status() == StatusCode::NOT_FOUND {
        get_static_file(uri).await
    } else {
        Ok(res)
    }
}

async fn get_static_file(uri: Uri) -> Result<Response<BoxBody>, (StatusCode, String)> {
    let req = Request::builder().uri(uri).body(Body::empty()).unwrap();

    // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
    match ServeDir::new("./dist/").oneshot(req).await {
        Ok(res) => Ok(res.map(boxed)),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", err),
        )),
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

        Ok(Response::new(body::boxed(body::Empty::new())))
    } else {
        tracing::warn!("CONNECT host is not socket addr: {:?}", req.uri());
        Ok((
            StatusCode::BAD_REQUEST,
            "CONNECT must be to a socket address",
        )
            .into_response())
    }
}

async fn tunnel(mut upgraded: Upgraded, addr: String) -> std::io::Result<()> {
    let mut server = TcpStream::connect(addr).await?;

    let (from_client, from_server) =
        tokio::io::copy_bidirectional(&mut upgraded, &mut server).await?;

    tracing::debug!(
        "client wrote {} bytes and received {} bytes",
        from_client,
        from_server
    );

    Ok(())
}
