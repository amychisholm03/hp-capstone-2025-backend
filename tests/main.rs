use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::sleep;

#[tokio::test]
async fn test_server_startup() {
    // Spawn the server in a background task
    let server = tokio::spawn(async {
        backend::run_server("localhost", "5040").await;
    });

    // Give the server a moment to start up
    sleep(Duration::from_secs(5)).await;

    // Try to connect to the server
    let addr = "localhost:5040";
    let stream = TcpStream::connect(addr).await;

    // Assert that the connection was successful
    assert!(stream.is_ok(), "Failed to connect to the server");

    server.abort();
}