use axum::{
    extract::Path,
    routing::{get, post, delete},
    Router,
};


pub fn build_routes() -> Router {
    let mut out = Router::new()
        .route("/", get(hello_world))
        .route("/PrintJob", post(post_print_job))
        .route("/PrintJob/{id}", delete(delete_print_job))
        .route("/Workflow", post(post_workflow))
        .route("/Workflow/{id}", delete(delete_workflow))
        .route("/SimulationReport", post(post_simulation_report))
        .route("/SimulationReport/{id}", delete(delete_simulation_report));

    
    let collections = ["PrintJob", "Workflow", "WorkflowStep", "SimulationReport"];
    for coll in collections {
        out = out.route(format!("/{coll}").as_str(), 
                get(|| get_coll(coll)))
            .route(format!("/{coll}/{{id}}").as_str(), 
                get(|path: Path<String>| get_id(path, coll)));
    }
    
    return out;
}


async fn hello_world() -> String {
    return "Hello, World".to_string();
}


async fn get_coll(coll: &str) -> String {
    return coll.to_string();
}


async fn get_id(Path(id): Path<String>, coll: &str) -> String {
    return format!("{coll} {id}");
}


async fn post_print_job() -> String {
    return "POST PrintJob".to_string();
}


async fn post_workflow() -> String {
    return "POST Workflow".to_string();
}


async fn post_simulation_report() -> String {
    return "POST SimulationReport".to_string();
}


async fn delete_print_job(Path(id): Path<String>) -> String {
    return format!("DELETE PrintJob {id}");
}


async fn delete_workflow(Path(id): Path<String>) -> String {
    return format!("DELETE PrintJob {id}");
}


async fn delete_simulation_report(Path(id): Path<String>) -> String {
    return format!("DELETE PrintJob {id}");
}