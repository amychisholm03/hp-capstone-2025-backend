use axum::http::StatusCode;
use reqwest;
use once_cell::sync::Lazy;
use tokio::sync::OnceCell;

const HOST: &str = "localhost";
const PORT: &str = "5040";

static SERVER: Lazy<OnceCell<()>> = Lazy::new(OnceCell::new);

async fn start_server() {
    SERVER.get_or_init(|| async {
        tokio::spawn(async {
            backend::run_server(HOST, PORT).await;
        });
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }).await;
}

#[tokio::test]
async fn test_hello_world() {
    start_server().await;

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("http://{}:{}/", HOST, PORT))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK.as_u16());
    let body = response.text().await.unwrap();
    assert_eq!(body, "Hello, World");
}

#[tokio::test]
async fn test_get_print_jobs() {
    start_server().await;

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
    start_server().await;

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
    start_server().await;

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("http://{}:{}/WorkflowStep", HOST, PORT))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK.as_u16());
}

#[tokio::test]
async fn test_get_simulation_reports() {
    start_server().await;

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("http://{}:{}/SimulationReport", HOST, PORT))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK.as_u16());
}

/*
#[tokio::test]
async fn test_get_print_job_by_id() {
    start_server().await;

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("http://{}:{}/PrintJob/1", HOST, PORT))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK.as_u16());
}

#[tokio::test]
async fn test_get_workflow_by_id() {
    start_server().await;

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("http://{}:{}/Workflow/1", HOST, PORT))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK.as_u16());
}

#[tokio::test]
async fn test_get_workflow_step_by_id() {
    start_server().await;

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("http://{}:{}/WorkflowStep/1", HOST, PORT))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK.as_u16());
}

#[tokio::test]
async fn test_get_simulation_report_by_id() {
    start_server().await;

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("http://{}:{}/SimulationReport/1", HOST, PORT))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK.as_u16());
}
*/
