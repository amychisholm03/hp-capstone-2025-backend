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


async fn get_print_jobs() -> String {
    return "PrintJob".to_string();
}


async fn get_workflows() -> String {
    return "Workflow".to_string();
}


async fn get_workflow_steps() -> String {
    return "WorkflowStep".to_string();
}


async fn get_simulation_reports() -> String {
    return "SimulationReport".to_string();
}


async fn get_print_job_by_id(Path(id): Path<String>) -> String {
    let id_u32: u32 = match id.parse() {
        Ok(val) => val,
        Err(_) => return format!("ID {id} invalid")
    };
    return match find_print_job(id_u32) {
        Some(val) => val,
        None => format!("PrintJob {id} not found")
    };
}


async fn get_workflow_by_id(Path(id): Path<String>) -> String {
    let id_u32: u32 = match id.parse() {
        Ok(val) => val,
        Err(_) => return format!("ID {id} invalid")
    };
    return match find_workflow(id_u32) {
        Some(val) => val,
        None => format!("Workflow {id} not found")
    };
}


async fn get_workflow_step_by_id(Path(id): Path<String>) -> String {
    let id_u32: u32 = match id.parse() {
        Ok(val) => val,
        Err(_) => return format!("ID {id} invalid")
    };
    return match find_workflow_step(id_u32) {
        Some(val) => val,
        None => format!("WorkflowStep {id} not found")
    };
}


async fn get_simulation_report_by_id(Path(id): Path<String>) -> String {
    let id_u32: u32 = match id.parse() {
        Ok(val) => val,
        Err(_) => return format!("ID {id} invalid")
    };
    return match find_simulation_report(id_u32) {
        Some(val) => val,
        None => format!("SimulationReport {id} not found")
    };
}


async fn post_print_job(Json(payload): Json<PrintJob>) -> String {
    return match insert_print_job(payload) {
        Some(val) => val.to_string(),
        None => "Did not insert".to_string()
    }
}


async fn post_workflow(Json(payload): Json<Workflow>) -> String {
    return match insert_workflow(payload) {
        Some(val) => val.to_string(),
        None => "Did not insert".to_string()
    }
}


async fn post_simulation_report(Json(payload): Json<SimulationReportArgs>) -> String {
    return match insert_simulation_report(payload) {
        Some(val) => val.to_string(),
        None => "Did not insert".to_string()
    }
}


async fn delete_print_job(Path(id): Path<String>) -> String {
    let id_u32: u32 = match id.parse() {
        Ok(val) => val,
        Err(_) => return format!("ID {id} invalid")
    };
    return match remove_print_job(id_u32) {
        Some(data) => data,
        None => "Failed to delete".to_string()
    }
}


async fn delete_workflow(Path(id): Path<String>) -> String {
    let id_u32: u32 = match id.parse() {
        Ok(val) => val,
        Err(_) => return format!("ID {id} invalid")
    };
    return match remove_workflow(id_u32) {
        Some(data) => data,
        None => "Failed to delete".to_string()
    }
}


async fn delete_simulation_report(Path(id): Path<String>) -> String {
    let id_u32: u32 = match id.parse() {
        Ok(val) => val,
        Err(_) => return format!("ID {id} invalid")
    };
    return match remove_simulation_report(id_u32) {
        Some(data) => data,
        None => "Failed to delete".to_string()
    }
}