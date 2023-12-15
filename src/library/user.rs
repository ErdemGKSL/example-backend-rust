use serde_json::{json, Value};

pub struct User {
  pub id: i32,
  pub name: String,
  pub email: String,
  pub password: String,
}

impl User {
  pub fn new(id: i32, name: String, email: String, password: String) -> Self {
    Self {
      id,
      name,
      email,
      password,
    }
  }

  pub fn to_json(&self) -> Value {
    json!({
      "id": self.id,
      "name": self.name,
      "email": self.email,
      "password": self.password
    })
  }
}

pub fn get_users() -> Vec<User> {
  vec![
      User::new(
        1,
        "Erdem".to_owned(),
        "erdem@gmail.com".to_owned(),
        "123456".to_owned(),
      ),
      User::new(
          2,
          "Ali".to_owned(),
          "ali@gmail.com".to_owned(),
          "123456".to_owned(),
      ),
      User::new(
          3,
          "Efe".to_owned(),
          "efe@gmail.com".to_owned(),
          "123456".to_owned(),
      ),
      User::new(
        4,
        "Arda".to_owned(),
        "arda@gmail.com".to_owned(),
        "123456".to_owned(),
      ),
      User::new(
        5,
        "Tuğkan".to_owned(),
        "tugkan@gmail.com".to_owned(),
        "123456".to_owned(),
      )
  ]
}