use std::env;

use crate::api::{*};
use crate::database::{*};
pub mod api;
pub mod database;

const HOST: &str = "0.0.0.0";
const PORT: &str = "80";


#[tokio::main]
async fn main() {
    // Use 'l' as an argument for local testing
    let args: Vec<String> = env::args().collect();
    let (host, port) = if args.len() > 1 && args[1] == "l" 
        { ("localhost", "5040") } else { (HOST, PORT) };

    // Initialize database
    // let db = Database::new();
    database_init();

    // Build Routes
    println!("Building Routes");
    let app = build_routes();

    // Run Server
    println!("Starting server on {host}:{port}");
    let listener = tokio::net::TcpListener::bind(format!("{host}:{port}")).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}