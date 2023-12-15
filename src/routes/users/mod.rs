use axum::{Router, routing::get};

pub mod get;

pub fn get_router() -> Router {
  Router::new()
    .route("/", get(get::trigger))
}