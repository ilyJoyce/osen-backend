use axum::{
    Router,
    extract::ConnectInfo,
    http::Request,
    body::Body,
    middleware::Next,
    response::Response,
    middleware,
};
use tower_http::services::{ ServeDir, ServeFile };
use std::net::SocketAddr;
use tokio;
use std::fs::{OpenOptions, create_dir_all};
use std::io::Write;
use axum::http::header::USER_AGENT;
use std::env;
use chrono::Local;

// Middleware to log requests
async fn log_middleware(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    req: Request<Body>,
    next: Next
) -> Response {
    // Get the request path and IP address
    let path = req.uri().path().to_string();
    let ip = addr.ip().to_string();

    // Get user agent from request headers
    let user_agent = req
        .headers()
        .get(USER_AGENT)
        .and_then(|ua| ua.to_str().ok())
        .unwrap_or("Unknown");

    // Get device type    
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

    // Get current datetime
    let now = Local::now();
    let datetime = now.format("%Y-%m-%d %H:%M:%S").to_string();

    println!("-----------------------------------------------------");
    // Print log to console
    println!(
        "[{}] Datei: \"{}\" an IP: \"{}\" gesendet. Gerät: \"{}\" (User-Agent: \"{}\")",
        datetime,
        path,
        ip,
        device_type,
        user_agent
    );
    println!("-----------------------------------------------------");

    // Prepare log line
    let log_line = format!(
        "[{}] || \"{}\" --> \"{}\" || Device: \"{}\" || User-Agent: \"{}\"\n",
        datetime,
        path,
        ip,
        device_type,
        user_agent
    );
    // Set log dir
    let log_dir = "log";
    // Create log dir if it doesn't exist
    create_dir_all(log_dir).unwrap();

    // Replace ':' in IP with '_' to avoid file system issues
    let safe_ip = ip.replace(':', "_");
    // Create a filename from IP
    let filename = format!("{}/{}.log", log_dir, safe_ip);

    // Open the file in append, create if doesn't exist
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)
        .unwrap();
    // Write the log line to the file
    file.write_all(log_line.as_bytes()).unwrap();

    next.run(req).await
}

// Main function
#[tokio::main]
async fn main() {
    // Print welcome message
    println!("Willkommen im OSEN Backend!");
    println!("Server wird gestartet - tokio::main");
    println!("-----------------------------------------------------");

    // Set up the static file server
    let static_service = ServeDir::new("..").not_found_service(ServeFile::new("../index.html"));

    // Create Axum router w/ static file service & logging middleware
    let app = Router::new()
        .fallback_service(static_service)
        .layer(middleware::from_fn_with_state((), log_middleware))
        .with_state(());

    // Default address and port
    let _addr = SocketAddr::from(([127, 0, 0, 1], 6464));

    // Parse command line arguments for port
    let mut port = 6464;

    // Check for --port argument
    let args: Vec<String> = env::args().collect();
    let mut i = 1;
    while i < args.len() {
        if args[i] == "--port" {
            if i + 1 < args.len() {
                if let Ok(p) = args[i + 1].parse::<u16>() {
                    port = p;
                } else {
                    eprintln!("Ungültiger Port: {}", args[i + 1]);
                    std::process::exit(1);
                }
                i += 1;
            }
        }
        i += 1;
    }

    // Set address with specified port
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    // Print server start message
    println!("Server gestartet auf http://{}", addr);
    println!("-----------------------------------------------------");

    // Start the server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}
