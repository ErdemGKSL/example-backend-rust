use axum::Router;
use tower_http::cors:: CorsLayer;

pub mod user;
pub mod users;

pub fn get_router() -> Router {
  Router::new()
    .nest("/user", user::get_router())
    .nest("/users", users::get_router())
    .layer(CorsLayer::very_permissive())
}