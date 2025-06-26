use axum::{
    Router, extract::ConnectInfo, http::Request, body::Body, middleware::Next, response::Response, 
    middleware,
};
use tower_http::services::{ServeDir, ServeFile};
use std::net::SocketAddr;
use tokio;
use std::fs::OpenOptions;
use std::io::Write;

async fn log_middleware(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    req: Request<Body>,
    next: Next,
) -> Response {
    let path = req.uri().path().to_string();
    let ip = addr.ip().to_string();

    println!("Datei: \"{}\" an IP: \"{}\" gesendet.", path, ip);

    let log_line = format!("\"{}\" --> \"{}\"\n", path, ip);
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("serve.log")
        .unwrap();
    file.write_all(log_line.as_bytes()).unwrap();

    next.run(req).await
}

#[tokio::main]
async fn main() {
    println!("Willkommen im OSEN Backend!");
    println!("Server wird gestartet - tokio::main");

    let static_service = ServeDir::new("..")
        .not_found_service(ServeFile::new("../index.html"));

    let app = Router::new()
        .fallback_service(static_service)
        .layer(middleware::from_fn_with_state((), log_middleware))
        .with_state(());

    let addr = SocketAddr::from(([127, 0, 0, 1], 6464));
    println!("Server gestartet auf http://{}", addr);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:6464").await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}