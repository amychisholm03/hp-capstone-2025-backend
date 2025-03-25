use axum::http::StatusCode;
use backend::database::*;
use reqwest;
use serde_json::{from_str, json};
use serial_test::serial;

const HOST: &str = "localhost";
const PORT: &str = "5040";

#[tokio::test]
#[serial]
async fn test_get_print_jobs() {
    // Start test server
    let server = tokio::spawn(async {
        backend::run_server(HOST, PORT).await;
    });
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

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
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

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
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

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
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

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
async fn test_simulation_report_create(){
    let server = tokio::spawn(async {
        backend::run_server(HOST, PORT).await;
    });
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    let rasterization_profile_id = test_get_rasterization_profile().await;
    let print_job_id = test_post_print_job(rasterization_profile_id).await;
    let workflow_id = test_post_workflow().await;
    let sim_report_id = test_post_simulation_report(print_job_id, workflow_id).await;
    test_get_simulation_report_by_id(sim_report_id).await;

    server.abort();
}

/// Test the full cycle of creating and deleting a print job
#[tokio::test]
#[serial]
async fn test_printjob_post_get_delete() {
    let server = tokio::spawn(async {
        backend::run_server(HOST, PORT).await;
    });
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    let rasterization_profile_id = test_get_rasterization_profile().await;
    let print_job_id = test_post_print_job(rasterization_profile_id).await;
    test_get_print_job_by_id(print_job_id).await;
    test_delete_print_job(print_job_id).await;

    server.abort();
}

#[tokio::test]
#[serial]
async fn test_workflow_post_get_delete(){
    let server = tokio::spawn(async {
        backend::run_server(HOST, PORT).await;
    });
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    let workflow_id = test_post_workflow().await;
    test_get_workflow_by_id(workflow_id).await;
    //test_delete_workflow(workflow_id).await;

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
        "Expected exactly five rasterization profiles from dummy-data.sql"
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
            { "WorkflowStepID": 0 },       // download file
            { "WorkflowStepID": 1 },         // preflight
            { "WorkflowStepID": 2 },         // impose
            { "WorkflowStepID": 3 },         // analyze
            { "WorkflowStepID": 4 },         // color setup
            { "WorkflowStepID": 5, "NumCores": 1 },         // rasterization
            { "WorkflowStepID": 6 }   // loading
        ]
    });

    let response = client
        .post(&format!("http://{}:{}/Workflow", HOST, PORT))
        .json(&payload)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED.as_u16());

    // Return workflow ID to use for future tests
    let body = response.text().await.unwrap();
    return body.parse::<DocID>().unwrap();
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

    match result {
        Err(e) => {
            panic!("Test failed due to error: {:?}", e);
        }
        Ok(response) => {
            assert_eq!(response.status(), StatusCode::CREATED.as_u16());

            // Set simulation report ID to use for future tests
            let body = response.text().await.unwrap();
            return body.parse::<DocID>().unwrap();
        }
    }
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
