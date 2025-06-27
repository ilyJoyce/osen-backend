use axum::{
    Router, extract::ConnectInfo, http::Request, body::Body, middleware::Next, response::Response, 
    middleware,
};
use tower_http::services::{ServeDir, ServeFile};
use std::net::SocketAddr;
use tokio;
use std::fs::OpenOptions;
use std::io::Write;
use axum::http::header::USER_AGENT;

async fn log_middleware(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    req: Request<Body>,
    next: Next,
) -> Response {
    let path = req.uri().path().to_string();
    let ip = addr.ip().to_string();

    // Extract User-Agent header
    let user_agent = req
        .headers()
        .get(USER_AGENT)
        .and_then(|ua| ua.to_str().ok())
        .unwrap_or("Unknown");

    // Simple device type detection
    let device_type = if user_agent.contains("iPhone") || user_agent.contains("iPad") {
        "iOS"
    } else if user_agent.contains("Android") {
        "Android"
    } else if user_agent.contains("Windows") {
        "Windows"
    } else if user_agent.contains("Macintosh") {
        "Mac"
    } else if user_agent.contains("Linux") {
        "Linux"
    } else {
        "Other"
    };

    println!(
        "Datei: \"{}\" an IP: \"{}\" gesendet. Gerät: \"{}\" (User-Agent: \"{}\")",
        path, ip, device_type, user_agent
    );

    let log_line = format!(
        "\"{}\" --> \"{}\" | Device: \"{}\" | User-Agent: \"{}\"\n",
        path, ip, device_type, user_agent
    );
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