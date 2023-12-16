use axum::extract::Json;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::library::user::{add_user, get_users};

pub async fn trigger(Json(user): Json<UserCreate>) -> Json<Value> {
    add_user(user.name, user.email, user.password);
    let user_opt = get_users().last();

    if let Some(user) = user_opt {
        Json(user.to_json())
    } else {
        unreachable!("User not found somehow, but added just now")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCreate {
    name: String,
    email: String,
    password: String,
}
