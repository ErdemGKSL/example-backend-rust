use axum::{Extension, Router};
use sqlx::PgPool;
use tower_http::cors:: CorsLayer;

pub mod user;
pub mod users;

pub fn get_router(db: PgPool) -> Router {
  Router::new()
    .layer(Extension(db))
    .layer(CorsLayer::very_permissive())
    .nest("/user", user::get_router())
    .nest("/users", users::get_router())
}