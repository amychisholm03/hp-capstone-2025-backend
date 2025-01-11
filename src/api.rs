use axum::{
    routing::get,
    Router,
};


pub fn build_routes() -> Router {
    return Router::new()
        .route("/", get(hello_world));
}


async fn hello_world() -> String {
    return "Hello, World".to_string();
}