use axum::http::StatusCode;
use reqwest;
use serde_json::json;
use serial_test::serial;

const HOST: &str = "localhost";
const PORT: &str = "5040";

/// A workflow with no workflow steps
/// should return a 422 status code.
#[tokio::test]
#[serial]
async fn test_post_empty_workflow() {
    let server = tokio::spawn(async {
        backend::run_server(HOST, PORT).await;
    });
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    let client = reqwest::Client::new();
    let payload = json!({
        "Title": "Test Workflow",
        "WorkflowSteps": []
    });

    let response = client
        .post(&format!("http://{}:{}/Workflow", HOST, PORT))
        .json(&payload)
        .send()
        .await
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
        "Workflow with no steps should return 422"
    );
    server.abort();
}

/// A workflow with a cyclic workflow step sequence
/// should return a 422 status code.
#[tokio::test]
#[serial]
async fn test_post_cyclic_workflow() {
    let server = tokio::spawn(async {
        backend::run_server(HOST, PORT).await;
    });
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    let client = reqwest::Client::new();
    let payload = json!({
        "Title": "Test Workflow",
        "WorkflowSteps": [
            { "WorkflowStepID": 1, "Prev": [2], "Next": [1] },
            { "WorkflowStepID": 2, "Prev": [0], "Next": [2] },
            { "WorkflowStepID": 3, "Prev": [1], "Next": [0] },
        ]
    });

    let response = client
        .post(&format!("http://{}:{}/Workflow", HOST, PORT))
        .json(&payload)
        .send()
        .await
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
        "Cyclic workflow should return 422"
    );
    server.abort();
}
