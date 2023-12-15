use axum::extract::{Json, Path};
use serde_json::{Value, json};

use crate::library::user::get_users;

pub async fn trigger(Path(user_id): Path<i32>) -> Json<Value> {
  let users = get_users();
  let user = users.iter().find(|user| user.id == user_id);

  if user.is_none() {
    return Json(json!({
      "error": "User not found"
    }));
  } else {
    return Json(json!(user.unwrap().to_json()))
  }
}