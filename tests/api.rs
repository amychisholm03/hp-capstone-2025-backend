use axum::http::StatusCode;
use backend::database::*;
use reqwest;
use serde_json::{from_str, json};
use serial_test::serial;

const HOST: &str = "localhost";
const PORT: &str = "5040";

#[tokio::test]
#[serial]
async fn test_hello_world() {
    // Start test server
    let server = tokio::spawn(async {
        backend::run_server(HOST, PORT).await;
    });
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("http://{}:{}/", HOST, PORT))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK.as_u16());
    let body = response.text().await.unwrap();
    assert_eq!(body, "Hello, World");

    server.abort();
}

#[tokio::test]
#[serial]
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
    server.abort();
}

#[tokio::test]
#[serial]
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
    server.abort();
}

#[tokio::test]
#[serial]
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
#[serial]
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
#[serial]
async fn test_all_post_get_then_delete() {
    let server = tokio::spawn(async {
        backend::run_server(HOST, PORT).await;
    });
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    let rasterization_profile_id = test_get_rasterization_profile().await;
    let print_job_id = test_post_print_job(rasterization_profile_id).await;
    let workflow_id = test_post_workflow().await;
    test_get_print_job_by_id(print_job_id).await;
    //test_get_workflow_by_id(workflow_id).await; TODO: currently broken because of find_workflow in database.rs
    //let sim_report_id = test_post_simulation_report(print_job_id, workflow_id).await;
    //test_get_simulation_report_by_id(sim_report_id).await;
    //test_delete_simulation_report(sim_report_id).await;
    test_delete_print_job(print_job_id).await;
    test_delete_workflow(workflow_id).await;

    server.abort();
}

async fn test_get_rasterization_profile() -> DocID {
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("http://{HOST}:{PORT}/RasterizationProfile"))
        .send()
        .await
        .unwrap();
    let list: Vec<RasterizationProfile> = from_str(&response.text().await.unwrap()).unwrap();

    assert!(
        list.len() == 5,
        "Expected at least five rasterization profiles from dummy-data.sql"
    );

    return list[0].id;
}

async fn test_post_print_job(rasterization_profile_id: DocID) -> DocID {
    let client = reqwest::Client::new();
    let payload = json!({
        "Title": "Test Print Job",
        "PageCount": 10,
        "RasterizationProfileID": rasterization_profile_id
    });

    let response = client
        .post(&format!("http://{}:{}/PrintJob", HOST, PORT))
        .json(&payload)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED.as_u16());

    // Set print job ID to use for future tests
    let body = response.text().await.unwrap();
    return body.parse::<DocID>().unwrap();
}

async fn test_post_workflow() -> DocID {
    let client = reqwest::Client::new();
    let payload = json!({
        "Title": "Test Workflow",
        "WorkflowSteps": [
            { "WorkflowStepID": 1, "Prev": [], "Next": [2] },
            { "WorkflowStepID": 2, "Prev": [1], "Next": [3] },
            { "WorkflowStepID": 3, "Prev": [2], "Next": [4] },
            { "WorkflowStepID": 4, "Prev": [3], "Next": [5] },
            { "WorkflowStepID": 5, "Prev": [4], "Next": [6] },
            { "WorkflowStepID": 6, "Prev": [5], "Next": [] }
        ]
    });

    let result = client
        .post(&format!("http://{}:{}/Workflow", HOST, PORT))
        .json(&payload)
        .send()
        .await;

    match result {
        Err(e) => {
            panic!("Test failed due to error: {:?}", e);    
        }
        Ok(response) => {
            println!("Response: {:?}", response);
            assert_eq!(response.status(), StatusCode::CREATED.as_u16());

            // Set workflow ID to use for future tests
            let body = response.text().await.unwrap();
            return body.parse::<DocID>().unwrap();
        }
    }
}

#[tokio::test]
#[serial]
async fn test_post_empty_workflow() {
    let server = tokio::spawn(async {
        backend::run_server(HOST, PORT).await;
    });
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

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
        StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        "Workflow with no steps should return 500"
    );
    server.abort();
}

/// A workflow with a cyclic workflow step sequence
/// should return an error.
#[tokio::test]
#[serial]
async fn test_post_cyclic_workflow() {
    let server = tokio::spawn(async {
        backend::run_server(HOST, PORT).await;
    });
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    let client = reqwest::Client::new();
    let payload = json!({
        "Title": "Test Workflow",
        "WorkflowSteps": [
            { "WorkflowStepID": 1, "Prev": [3], "Next": [2] },
            { "WorkflowStepID": 2, "Prev": [1], "Next": [3] },
            { "WorkflowStepID": 3, "Prev": [2], "Next": [1] },
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
        StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        "Cyclic workflow should return 500"
    );
    server.abort();
}

async fn test_post_simulation_report(print_job_id: DocID, workflow_id: DocID) -> DocID {
    let client = reqwest::Client::new();
    let payload = json!({
        "PrintJobID": print_job_id,
        "WorkflowID": workflow_id,
    });

    let result = client
        .post(&format!("http://{}:{}/SimulationReport", HOST, PORT))
        .json(&payload)
        .send()
        .await;

    if let Err(e) = result {
        println!("Error posting simulation report: {:?}", e);
        panic!("Test failed due to error");
    }
    let response = result.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED.as_u16());

    // Set simulation report ID to use for future tests
    let body = response.text().await.unwrap();
    return body.parse::<DocID>().unwrap();
}

async fn test_get_print_job_by_id(print_job_id: DocID) {
    let client = reqwest::Client::new();
    let response = client
        .get(&format!(
            "http://{}:{}/PrintJob/{}",
            HOST, PORT, print_job_id
        ))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK.as_u16());
}

async fn test_get_workflow_by_id(workflow_id: DocID) {
    let client = reqwest::Client::new();
    let response = client
        .get(&format!(
            "http://{}:{}/Workflow/{}",
            HOST, PORT, workflow_id
        ))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK.as_u16());
}

/*
#[tokio::test]
#[serial]
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
    */

async fn test_get_simulation_report_by_id(sim_report_id: DocID) {
    let client = reqwest::Client::new();
    let response = client
        .get(&format!(
            "http://{}:{}/SimulationReport/{}",
            HOST, PORT, sim_report_id
        ))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK.as_u16());
}

async fn test_delete_print_job(print_job_id: DocID) {
    let client = reqwest::Client::new();
    let response = client
        .delete(&format!(
            "http://{}:{}/PrintJob/{}",
            HOST, PORT, print_job_id
        ))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT.as_u16());
}

async fn test_delete_workflow(workflow_id: DocID) {
    let client = reqwest::Client::new();
    let response = client
        .delete(&format!(
            "http://{}:{}/Workflow/{}",
            HOST, PORT, workflow_id
        ))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT.as_u16());
}

async fn test_delete_simulation_report(sim_report_id: DocID) {
    let client = reqwest::Client::new();
    let response = client
        .delete(&format!(
            "http://{}:{}/SimulationReport/{}",
            HOST, PORT, sim_report_id
        ))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT.as_u16());
}
