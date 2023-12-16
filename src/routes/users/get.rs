use axum::extract::Json;
use serde_json::{json, Value};

use crate::library::user;

pub async fn trigger() -> Json<Value> {
    let users = user::get_users();

    Json(json!(users .iter().filter(|user| user.id > 0).map(|user| user.to_json()).collect::<Vec<Value>>()))
}