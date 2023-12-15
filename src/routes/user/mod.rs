use axum::Router;

mod id;

pub fn get_router() -> Router {
  Router::new()
    .nest("/:user_id", id::get_router())
}