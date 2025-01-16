use axum::http::StatusCode;
use reqwest;

const HOST: &str = "localhost";
const PORT: &str = "5040";

#[tokio::test]
async fn test_hello_world() {
    // Start test server
    let server = tokio::spawn(async {
        backend::run_server(HOST, PORT).await;
    });
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    // Start mock client
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("http://{}:{}/", HOST, PORT))
        .send()
        .await
        .unwrap();

    // Check response
    assert_eq!(response.status(), StatusCode::OK.as_u16());
    let body = response.text().await.unwrap();
    assert_eq!(body, "Hello, World");

    // Kill test server
    server.abort();
}

#[tokio::test]
async fn test_get_print_jobs() {
    // Start test server
    let server = tokio::spawn(async {
        backend::run_server(HOST, PORT).await;
    });
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("http://{}:{}/PrintJob", HOST, PORT))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK.as_u16());
}

#[tokio::test]
async fn test_get_workflows() {
    // Start test server
    let server = tokio::spawn(async {
        backend::run_server(HOST, PORT).await;
    });
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("http://{}:{}/Workflow", HOST, PORT))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK.as_u16());
    
}

#[tokio::test]
async fn test_get_workflow_steps() {
    // Start test server
    let server = tokio::spawn(async {
        backend::run_server(HOST, PORT).await;
    });
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("http://{}:{}/WorkflowStep", HOST, PORT))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK.as_u16());
    server.abort();
}

#[tokio::test]
async fn test_get_simulation_reports() {
    // Start test server
    let server = tokio::spawn(async {
        backend::run_server(HOST, PORT).await;
    });
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("http://{}:{}/SimulationReport", HOST, PORT))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK.as_u16());
    server.abort();
}

#[tokio::test]
async fn test_get_print_job_by_id() {
    // Start test server
    let server = tokio::spawn(async {
        backend::run_server(HOST, PORT).await;
    });
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("http://{}:{}/PrintJob/1", HOST, PORT))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK.as_u16());
    server.abort();
}

#[tokio::test]
async fn test_get_workflow_by_id() {
    // Start test server
    let server = tokio::spawn(async {
        backend::run_server(HOST, PORT).await;
    });
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("http://{}:{}/Workflow/1", HOST, PORT))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK.as_u16());
    server.abort();
}

#[tokio::test]
async fn test_get_workflow_step_by_id() {
    // Start test server
    let server = tokio::spawn(async {
        backend::run_server(HOST, PORT).await;
    });
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("http://{}:{}/WorkflowStep/1", HOST, PORT))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK.as_u16());
    server.abort();
}

#[tokio::test]
async fn test_get_simulation_report_by_id() {
    // Start test server
    let server = tokio::spawn(async {
        backend::run_server(HOST, PORT).await;
    });
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("http://{}:{}/SimulationReport/1", HOST, PORT))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK.as_u16());
    server.abort();
}
