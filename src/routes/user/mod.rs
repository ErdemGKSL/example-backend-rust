use axum::{routing::post, Router};

mod id;
mod post;

pub fn get_router() -> Router {
    Router::new()
        .nest("/:user_id", id::get_router())
        .route("/", post(post::trigger))
}