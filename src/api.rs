use std::time::{SystemTime, UNIX_EPOCH};
use crate::database::*;
use axum::{
    extract::Path,
    response::{Response, IntoResponse},
    routing::{delete, get, post},
    Json, Router,
};
use http::Method;
use hyper::StatusCode;
use serde_json::json;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

/// Builds the routes for the API.
pub fn build_routes() -> Router {
    // https://dev.to/amaendeepm/api-development-in-rust-cors-tower-middleware-and-the-power-of-axum-397k
    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_headers(Any);

    return Router::new()
        .route("/", get(hello_world))
        // Error logging route(s)
        .route("/ErrorLog", get(get_errors_detailed))
        // PrintJob Routes
        .route("/PrintJob", get(get_print_jobs))
        .route("/PrintJob", post(post_print_job))
        .route("/PrintJob/{id}", get(get_print_job_by_id))
        .route("/PrintJob/{id}", delete(delete_print_job))
        // Rasterization Profile Routes
        .route("/RasterizationProfile", get(get_rasterization_profiles))
        .route(
            "/RasterizationProfile/{id}",
            get(get_rasterization_profile_by_id),
        )
        .route("/RasterizationProfile", post(post_rasterization_profile))
        .route(
            "/RasterizationProfile/{id}",
            delete(delete_rasterization_profile),
        )
        // Workflow Routes
        .route("/Workflow", get(get_workflows))
        .route("/Workflow", post(post_workflow))
        .route("/Workflow/{id}", get(get_workflow_by_id))
        .route("/Workflow/{id}", delete(delete_workflow))
        // WorkflowStep Routes
        .route("/WorkflowStep", get(get_workflow_steps))
        .route("/WorkflowStep/{id}", get(get_workflow_step_by_id))
        // SimulationReport Routes
        .route("/SimulationReport", get(get_simulation_reports))
        .route("/SimulationReport", post(post_simulation_report))
        .route("/SimulationReport/{id}", get(get_simulation_report_by_id))
        .route("/SimulationReport/{id}", delete(delete_simulation_report))
        
        // CORS
        .layer(ServiceBuilder::new().layer(cors_layer));
}

/// Returns a sanitized tuple of the response code and message.
///
/// ### Arguments
/// * `code` - The HTTP status code to return.
/// * `message` - The message to return.
///
/// ### Returns
/// A tuple of the HTTP status code and message.
fn response(code: u16, message: String) -> Response {
    return (StatusCode::from_u16(code).unwrap(), message).into_response();
}

/// Returns a sanitized tuple of the response code and message
/// alongside inserting the error into the database. 
/// !! Must be awaited !!
///
/// ### Arguments
/// * `code` - The HTTP status code to return / insert.
/// * `message` - The message to return / insert.
/// * `domain` - The domain (address) to insert, 
///   concatinated with 'api.wsuv-hp-capstone.com/'
/// * `method` - The method (GET|POST|DELETE) to insert.
/// * `request` - The original request to insert.
///
/// ### Returns
/// A tuple of the HTTP status code and message.
///
/// ### Inserts
/// A detailed error message into the database.
async fn error_response(code: u16, message: String, domain: String, method:  String, request: String) -> Response {
    let _ = insert_error_detailed(ErrorDetailed::new(
                SystemTime::now().duration_since(UNIX_EPOCH).expect("Issue discerning current time.").as_secs() as u32,
                code as u32, 
                "api.wsuv-hp-capstone.com/".to_string() + &domain,
                request,
                method,
                message.clone(),
            )).await;

    return (StatusCode::from_u16(code).unwrap(), message).into_response();
}

async fn hello_world() -> String {
    return "Hello, World".to_string();
}
        
async fn get_errors_detailed() -> Response {
    return match query_errors_detailed().await {
        Ok(data) => response(200, json!(data).to_string()),
        Err(_) => error_response(
            400, "An error occured.".to_string(),
            "/ErrorLog".to_string(), "GET".to_string(),
            "".to_string(),
        ).await,
    };
}

async fn get_print_jobs() -> Response {
    return match query_print_jobs().await {
        Ok(data) => response(200, json!(data).to_string()),
        Err(_) => error_response(
            400, "Invalid Query".to_string(),
            "/PrintJob".to_string(), "GET".to_string(),
            "".to_string(),
        ).await,
    };
}

async fn get_rasterization_profiles() -> Response {
    return match query_rasterization_profiles().await {
        Ok(data) => response(200, json!(data).to_string()),
        Err(_) => error_response(
            400, "An error occurred.".to_string(),
            "/RasterizationProfile".to_string(), "GET".to_string(),
            "".to_string(),
        ).await,
    };
}

async fn get_workflows() -> Response {
    return match query_workflows().await {
        Ok(data) => response(200, json!(data).to_string()),
        Err(_) => error_response(
            400, "Invalid Query".to_string(),
            "/Workflow".to_string(), "GET".to_string(),
            "".to_string(),
        ).await,
    };
}

async fn get_workflow_steps() -> Response {
    return match query_workflow_steps().await {
        Ok(data) => response(200, json!(data).to_string()),
        Err(_) => error_response(
            400, "An error occurred.".to_string(),
            "/WorkflowStep".to_string(), "GET".to_string(),
            "".to_string(),
        ).await,
    };
}

async fn get_simulation_reports() -> Response {
    return match query_simulation_reports().await {
        Ok(data) => response(200, json!(data).to_string()),
        Err(_) => error_response(
            400, "An error occurred.".to_string(),
            "/SimulationReport".to_string(), "GET".to_string(),
            "".to_string(),
        ).await,
    };
}

/// Returns a PrintJob by its ID.
///
/// ### Arguments
/// * `id_str` - The ID of the PrintJob to return.
///
/// ### Returns
/// The PrintJob with the given ID.
async fn get_print_job_by_id(Path(id_str): Path<String>) -> Response {
    let id: DocID = match id_str.parse() {
        Ok(data) => data,
        Err(_) => return response(400, format!("Invalid ID: {id_str}")),
    };
    return match find_print_job(id).await {
        Ok(data) => response(200, json!(data).to_string()),
        Err(_) => error_response(
            404, format!("PrintJob not found: {id_str}"),
            format!("/PrintJob/{id_str}"), "GET".to_string(),
            "".to_string(),
        ).await,
    };
}

async fn get_rasterization_profile_by_id(Path(id_str): Path<String>) -> Response {
    let id: DocID = match id_str.parse() {
        Ok(data) => data,
        Err(_) => return error_response(
            400, format!("Invalid ID: {id_str}"),
            format!("/RasterizationProfile/{id_str}"), "GET".to_string(),
            "".to_string(),
        ).await,
    };
    return match find_rasterization_profile(id).await {
        Ok(data) => response(200, json!(data).to_string()),
        Err(_) => error_response(
            404, "An error occurred.".to_string(),
            format!("/RasterizationProfile/{id_str}"), "GET".to_string(),
            "".to_string(),
        ).await,
    };
}

/// Returns a Workflow by its ID.
///
/// ### Arguments
/// * `id_str` - The ID of the Workflow to return.
///
/// ### Returns
/// The Workflow with the given ID.
async fn get_workflow_by_id(Path(id_str): Path<String>) -> Response {
    let id: DocID = match id_str.parse() {
        Ok(data) => data,
        Err(_) => return error_response(
            400, format!("Invalid ID: {id_str}"),
            format!("/Workflow/{id_str}"), "GET".to_string(),
            "".to_string(),
        ).await,
    };
    return match find_workflow(id).await {
        Ok(data) => response(200, json!(data).to_string()),
        Err(e) => error_response(
            500, e.to_string(),
            format!("/Workflow/{id_str}"), "GET".to_string(),
            "".to_string(),
        ).await,
    };
}

/// Returns a Workflow Step by its ID.
///
/// ### Arguments
/// * `id_str` - The ID of the Workflow Step to return.
///
/// ### Returns
/// The Workflow Step with the given ID.
async fn get_workflow_step_by_id(Path(id_str): Path<String>) -> Response {
    let id: DocID = match id_str.parse() {
        Ok(data) => data,
        Err(_) => return error_response(
            400, format!("Invalid ID: {id_str}"),
            format!("/WorkflowStep/{id_str}"), "GET".to_string(),
            "".to_string(),
        ).await,
    };
    return match find_workflow_step(id).await {
        Ok(data) => response(200, json!(data).to_string()),
        Err(_) => error_response(
            404, format!("WorkflowStep not found: {id_str}"),
            format!("/WorkflowStep/{id_str}"), "GET".to_string(),
            "".to_string(),
        ).await,
    };
}

/// Returns a Simulation Report by its ID.
///
/// ### Arguments
/// * `id_str` - The ID of the Simulation Report to return.
///
/// ### Returns
/// The Simulation Report with the given ID.
async fn get_simulation_report_by_id(Path(id_str): Path<String>) -> Response {
    let id: DocID = match id_str.parse() {
        Ok(data) => data,
        Err(_) => return error_response(
            400, format!("Invalid ID: {id_str}"),
            format!("/SimulationReport/{id_str}"), "GET".to_string(),
            "".to_string(),
        ).await,
    };
    return match find_simulation_report(id).await {
        Ok(data) => response(200, json!(data).to_string()),
        Err(_) => error_response(
            404, format!("SimulationReport not found: {id_str}"),
            format!("/SimulationReport/{id_str}"), "GET".to_string(),
            "".to_string(),
        ).await,
    };
}

async fn post_rasterization_profile(Json(payload): Json<RasterizationProfile>) -> Response {
    return match insert_rasterization_profile(payload.clone()).await {
        Ok(data) => response(201, data.to_string()),
        Err(_) => error_response(
            500, "Failed to insert rasterization profile".to_string(),
            "/RasterizationProfile".to_string(), "POST".to_string(),
            serde_json::to_string(&payload).unwrap_or("".to_string()),
        ).await, 
    };
}

/// Inserts a Print Job into the database.
///
/// ### Arguments
/// * `payload` - A JSON object of a Print Job to insert.
///
/// ### Returns
/// The status code of the insertion.
async fn post_print_job(Json(payload): Json<PrintJob>) -> Response {
    return match insert_print_job(payload.clone()).await {
        Ok(data) => response(201, data.to_string()),
        Err(_) => error_response(
            500, "Failed to insert print job".to_string(),
            "/PrintJob".to_string(), "POST".to_string(),
            serde_json::to_string(&payload).unwrap_or("".to_string()),
        ).await,     
    };
}

async fn post_workflow(Json(payload): Json<WorkflowArgs>) -> Response {
    return match insert_workflow(payload.clone()).await {
        Ok(data) => response(201, data.to_string()),
        Err(err) => {
            let code: u16 = match err.to_string().to_ascii_lowercase() == "invalid workflow" {
                true => 422,
                false => {println!("Error: {}", err); 500}
            }; 
            error_response(
                code, err.to_string(),
                "/Workflow".to_string(), "POST".to_string(),
                serde_json::to_string(&payload).unwrap_or("".to_string()),
            ).await
        } 
    };
}

/// Inserts a Simulation Report into the database.
///
/// ### Arguments
/// * `payload` - A JSON object of a Simulation Report to insert.
///
/// ### Returns
/// The status code of the insertion.
async fn post_simulation_report(Json(payload): Json<SimulationReportArgs>) -> Response {
    return match insert_simulation_report(payload.PrintJobID, payload.WorkflowID).await {
        Ok(data) => response(201, data.to_string()),
        Err(err) => error_response(
            500, err.to_string(),
            "/SimulationReport".to_string(), "POST".to_string(),
            serde_json::to_string(&payload).unwrap_or("".to_string()),
        ).await, 
    };
}

/// Deletes a Print Job from the database.
///
/// ### Arguments
/// * `id_str` - The ID of the Print Job to delete.
///
/// ### Returns
/// The status code of the deletion.
async fn delete_print_job(Path(id_str): Path<String>) -> Response {
    let id: DocID = match id_str.parse() {
        Ok(data) => data,
        Err(_) => return response(400, format!("Invalid ID: {id_str}")),
    };
    return match remove_print_job(id).await {
        Ok(_data) => response(204, "".to_string()), //TODO: Return the deleted data?
        Err(_) => response(404, format!("PrintJob not found: {id_str}")), //TODO: Need to handle error 409(conflict) if the printjob can't be deleted
    };
}

async fn delete_rasterization_profile(Path(id_str): Path<String>) -> Response {
    let id: DocID = match id_str.parse() {
        Ok(data) => data,
        Err(_) => return response(400, format!("Invalid ID: {id_str}")),
    };
    return match remove_rasterization_profile(id).await {
        Ok(_data) => response(204, "".to_string()), //TODO: Return the deleted data?
        Err(_) => response(404, format!("Unable to delete.")),
    };
}

/// Deletes a Workflow from the database.
///
/// ### Arguments
/// * `id_str` - The ID of the Workflow to delete.
///
/// ### Returns
/// The status code of the deletion.
async fn delete_workflow(Path(id_str): Path<String>) -> Response {
    let id: DocID = match id_str.parse() {
        Ok(data) => data,
        Err(_) => return response(400, format!("Invalid ID: {id_str}")),
    };
    return match remove_workflow(id).await {
        Ok(_data) => response(204, "".to_string()), //TODO: Return the deleted data?
        Err(_) => response(404, format!("Unable to delete.")), //TODO: Need to handle error 409(conflict) if the workflow can't be deleted
    };
}

/// Deletes a Simulation Report from the database.
///
/// ### Arguments
/// * `id_str` - The ID of the Simulation Report to delete.
///
/// ### Returns
/// The status code of the deletion.
async fn delete_simulation_report(Path(id_str): Path<String>) -> Response {
    let id: DocID = match id_str.parse() {
        Ok(data) => data,
        Err(_) => return response(400, format!("Invalid ID: {id_str}")),
    };
    return match remove_simulation_report(id).await {
        Ok(_data) => response(204, "".to_string()), //TODO: Return the deleted data?
        Err(_) => response(404, format!("SimulationReport not found: {id_str}")),
    };
}
