use axum::{
    extract::Path,
    routing::{get, post, delete},
    Router,
    Json
};
use crate::database::{*};


pub fn build_routes() -> Router {
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
        .route("/SimulationReport/{id}", delete(delete_simulation_report));
}


async fn hello_world() -> String {
    return "Hello, World".to_string();
}


// TODO: Update to allow for querying
async fn get_print_jobs() -> String {
    return match query_print_jobs() {
        Some(data) => data,
        None => "Invalid Query".to_string()
    }
}


// TODO: Update to allow for querying
async fn get_workflows() -> String {
    return match query_workflows() {
        Some(data) => data,
        None => "Invalid Query".to_string()
    }
}


// TODO: Update to allow for querying
async fn get_workflow_steps() -> String {
    return match query_workflow_steps() {
        Some(data) => data,
        None => "Invalid Query".to_string()
    }
}


// TODO: Update to allow for querying
async fn get_simulation_reports() -> String {
    return match query_simulation_reports() {
        Some(data) => data,
        None => "Invalid Query".to_string()
    }
}


async fn get_print_job_by_id(Path(id_str): Path<String>) -> String {
    let id: DocID = match id_str.parse() {
        Ok(data) => data,
        Err(_) => return format!("ID {id_str} invalid")
    };
    return match find_print_job(id) {
        Some(data) => data,
        None => format!("PrintJob {id_str} not found")
    };
}


async fn get_workflow_by_id(Path(id_str): Path<String>) -> String {
    let id: DocID = match id_str.parse() {
        Ok(data) => data,
        Err(_) => return format!("ID {id_str} invalid")
    };
    return match find_workflow(id) {
        Some(data) => data,
        None => format!("Workflow {id_str} not found")
    };
}


async fn get_workflow_step_by_id(Path(id_str): Path<String>) -> String {
    let id: DocID = match id_str.parse() {
        Ok(data) => data,
        Err(_) => return format!("ID {id_str} invalid")
    };
    return match find_workflow_step(id) {
        Some(data) => data,
        None => format!("WorkflowStep {id_str} not found")
    };
}


async fn get_simulation_report_by_id(Path(id_str): Path<String>) -> String {
    let id: DocID = match id_str.parse() {
        Ok(data) => data,
        Err(_) => return format!("ID {id_str} invalid")
    };
    return match find_simulation_report(id) {
        Some(data) => data,
        None => format!("SimulationReport {id_str} not found")
    };
}


async fn post_print_job(Json(payload): Json<PrintJob>) -> String {
    return match insert_print_job(payload) {
        Some(data) => data.to_string(),
        None => "Did not insert".to_string()
    }
}


async fn post_workflow(Json(payload): Json<Workflow>) -> String {
    return match insert_workflow(payload) {
        Some(data) => data.to_string(),
        None => "Did not insert".to_string()
    }
}


async fn post_simulation_report(Json(payload): Json<SimulationReportArgs>) -> String {
    return match insert_simulation_report(payload) {
        Some(data) => data.to_string(),
        None => "Did not insert".to_string()
    }
}


async fn delete_print_job(Path(id_str): Path<String>) -> String {
    let id: DocID = match id_str.parse() {
        Ok(data) => data,
        Err(_) => return format!("ID {id_str} invalid")
    };
    return match remove_print_job(id) {
        Some(data) => data,
        None => "Failed to delete".to_string()
    }
}


async fn delete_workflow(Path(id_str): Path<String>) -> String {
    let id: DocID = match id_str.parse() {
        Ok(data) => data,
        Err(_) => return format!("ID {id_str} invalid")
    };
    return match remove_workflow(id) {
        Some(data) => data,
        None => "Failed to delete".to_string()
    }
}


async fn delete_simulation_report(Path(id_str): Path<String>) -> String {
    let id: DocID = match id_str.parse() {
        Ok(data) => data,
        Err(_) => return format!("ID {id_str} invalid")
    };
    return match remove_simulation_report(id) {
        Some(data) => data,
        None => "Failed to delete".to_string()
    }
}