use axum::{Router, routing::get};

pub fn server_routes() -> Router {
    Router::new().route("/a", get(home))
}

pub async fn home() -> String {
    "Hello World!".to_string()
}
