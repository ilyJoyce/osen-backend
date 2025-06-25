use axum::{
    routing::get,
    Router,
};
use tower_http::services::{ServeDir, ServeFile};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let static_service = ServeDir::new("..")
        .not_found_service(ServeFile::new("../index.html"));

    let app = Router::new()
        .fallback_service(static_service);

    let addr = SocketAddr::from(([127, 0, 0, 1], 6464));
    println!("Server gestartet auf http://{}", addr);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:6464").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
