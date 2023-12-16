use axum::extract::Json;
use serde_json::Value;
use serde::{Deserialize, Serialize};

use crate::library::user::{add_user, get_users};

pub async fn trigger(Json(user): Json<UserCreate>) -> Json<Value> {
  add_user(user.name, user.email, user.password);

  return Json(get_users().last().unwrap().to_json());
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCreate {
  name: String,
  email: String,
  password: String,
}