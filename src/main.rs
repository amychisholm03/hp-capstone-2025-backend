use std::env;
use std::process;
use crate::api::{*};
use crate::database::{*};
pub mod api;
pub mod database;
pub mod simulation;

const HOST: &str = "0.0.0.0";
const PORT: &str = "80"; // production port

// Runs the server, allowing reuse in tests
pub async fn run_server(host: &str, port: &str) {
    // Initialize database
    database_init();

    // Build Routes
    println!("Building Routes");
    let app = build_routes();

    // Run Server
    println!("Starting server on {host}:{port}");
    let listener = match tokio::net::TcpListener::bind(format!("{host}:{port}")).await {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Failed to bind TCP listener: {}", e);
            process::exit(1);
        }
    };

    if let Err(e) = axum::serve(listener, app).await {
        eprintln!("Failed to serve: {}", e);
        process::exit(1);
    }
}

#[tokio::main]
pub async fn main() {
    // Use 'l' as an argument for local testing
    let args: Vec<String> = env::args().collect();
    let (host, port) = if args.len() > 1 && args[1] == "l" {
        ("localhost", "5040")
    } else {
        (HOST, PORT)
    };

    run_server(host, port).await;
}