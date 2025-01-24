use axum::{
    extract::Path,
    routing::{get, post, delete},
    response::IntoResponse,
    Router,
    Json
};
use serde_json::json;
use http::Method;
use tower::ServiceBuilder;
use tower_http::cors::{CorsLayer, Any};
use hyper::StatusCode;
use crate::database::{*};


pub fn build_routes() -> Router {
    // https://dev.to/amaendeepm/api-development-in-rust-cors-tower-middleware-and-the-power-of-axum-397k
    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_headers(Any);

    return Router::new()
        .route("/", get(hello_world))
        // PrintJob Routes
        .route("/PrintJob", get(get_print_jobs))
        .route("/PrintJob", post(post_print_job))
        .route("/PrintJob/{id}", get(get_print_job_by_id))
        .route("/PrintJob/{id}", delete(delete_print_job))
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


fn response(code: u16, message: String) -> impl IntoResponse {
    return (StatusCode::from_u16(code).unwrap(), message);
}


async fn hello_world() -> String {
    return "Hello, World".to_string();
}


// TODO: Update to allow for querying
async fn get_print_jobs() -> impl IntoResponse {
    return match query_print_jobs().await {
        Ok(data) => response(200, json!(data).to_string()),
        Err(e) => response(400, e)
    }
}


// TODO: Update to allow for querying
async fn get_workflows() -> impl IntoResponse {
    return match query_workflows().await {
        Ok(data) => response(200, json!(data).to_string()),
        Err(_) => response(400, "Invalid Query".to_string())
    }
}


// TODO: Update to allow for querying
async fn get_workflow_steps() -> impl IntoResponse {
    return match query_workflow_steps().await {
        Ok(data) => response(200, json!(data).to_string()),
        Err(_) => response(400, "Invalid Query".to_string())
    }
}


// TODO: Update to allow for querying
async fn get_simulation_reports() -> impl IntoResponse {
    return match query_simulation_reports().await {
        Ok(data) => response(200, json!(data).to_string()),
        Err(_) => response(400, "Invalid Query".to_string())
    }
}


async fn get_print_job_by_id(Path(id_str): Path<String>) -> impl IntoResponse {
    let id: DocID = match id_str.parse() {
        Ok(data) => data,
        Err(_) => return response(400, format!("Invalid ID: {id_str}"))
    };
    return match find_print_job(id).await {
        Ok(data) => response(200, json!(data).to_string()),
        Err(_) => response(404, format!("PrintJob not found: {id_str}"))
    };
}


async fn get_workflow_by_id(Path(id_str): Path<String>) -> impl IntoResponse {
    let id: DocID = match id_str.parse() {
        Ok(data) => data,
        Err(_) => return response(400, format!("Invalid ID: {id_str}"))
    };
    return match find_workflow(id).await {
        Ok(data) => response(200, json!(data).to_string()),
        Err(_) => response(404, format!("Workflow not found: {id_str}"))
    };
}


async fn get_workflow_step_by_id(Path(id_str): Path<String>) -> impl IntoResponse {
    let id: DocID = match id_str.parse() {
        Ok(data) => data,
        Err(_) => return response(400, format!("Invalid ID: {id_str}"))
    };
    return match find_workflow_step(id).await {
        Ok(data) => response(200, json!(data).to_string()),
        Err(_) => response(404, format!("WorkflowStep not found: {id_str}"))
    };
}


async fn get_simulation_report_by_id(Path(id_str): Path<String>) -> impl IntoResponse {
    let id: DocID = match id_str.parse() {
        Ok(data) => data,
        Err(_) => return response(400, format!("Invalid ID: {id_str}"))
    };
    return match find_simulation_report(id).await {
        Ok(data) => response(200, json!(data).to_string()),
        Err(_) => response(404, format!("SimulationReport not found: {id_str}"))
    };
}


async fn post_print_job(Json(payload): Json<PrintJob>) -> impl IntoResponse {
    return match insert_print_job(payload).await {
        Ok(data) => response(201, data.to_string()),
        Err(_) => response(500, "Failed to insert".to_string()) //TODO: Better error code/message? What would cause this?
    }
}


async fn post_workflow(Json(payload): Json<Workflow>) -> impl IntoResponse {
    return match insert_workflow(payload).await {
        Ok(data) => response(201, data.to_string()),
        Err(err) => response(500, err.to_string()) //TODO: Better error code/message? What would cause this?
    }
}


async fn post_simulation_report(Json(payload): Json<SimulationReportArgs>) -> impl IntoResponse {
    return match insert_simulation_report(payload.PrintJobID, payload.WorkflowID).await {
        Ok(data) => response(201, data.to_string()),
        Err(_) => response(500, "Failed to insert".to_string()) //TODO: Better error code/message? What would cause this?
    }
}


async fn delete_print_job(Path(id_str): Path<String>) -> impl IntoResponse {
    let id: DocID = match id_str.parse() {
        Ok(data) => data,
        Err(_) => return response(400, format!("Invalid ID: {id_str}"))
    };
    return match remove_print_job(id).await {
        Ok(_data) => response(204, "".to_string()), //TODO: Return the deleted data?
        Err(_) => response(404, format!("PrintJob not found: {id_str}"))
        //TODO: Need to handle error 409(conflict) if the printjob can't be deleted
    }
}


async fn delete_workflow(Path(id_str): Path<String>) -> impl IntoResponse {
    let id: DocID = match id_str.parse() {
        Ok(data) => data,
        Err(_) => return response(400, format!("Invalid ID: {id_str}"))
    };
    return match remove_workflow(id).await {
        Ok(_data) => response(204, "".to_string()), //TODO: Return the deleted data?
        Err(_) => response(404, format!("Workflow not found: {id_str}"))
        //TODO: Need to handle error 409(conflict) if the workflow can't be deleted
    }
}


async fn delete_simulation_report(Path(id_str): Path<String>) -> impl IntoResponse {
    let id: DocID = match id_str.parse() {
        Ok(data) => data,
        Err(_) => return response(400, format!("Invalid ID: {id_str}"))
    };
    return match remove_simulation_report(id).await {
        Ok(_data) => response(204, "".to_string()), //TODO: Return the deleted data?
        Err(_) => response(404, format!("SimulationReport not found: {id_str}"))
    }
}
