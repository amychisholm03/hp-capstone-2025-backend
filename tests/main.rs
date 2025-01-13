use tokio::net::TcpStream;
use std::time::Duration;
use tokio::time::sleep;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_server_startup() {
        // Spawn the server in a background task
        tokio::spawn(async {
            backend::main();
        });

        // Give the server a moment to start up
        sleep(Duration::from_secs(5)).await;

        // Try to connect to the server
        let addr = "0.0.0.0:80";
        let stream = TcpStream::connect(addr).await;

        // Assert that the connection was successful
        assert!(stream.is_ok(), "Failed to connect to the server");
    }
}