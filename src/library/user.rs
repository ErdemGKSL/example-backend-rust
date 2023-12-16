use serde_json::{json, Value};

static mut USERS: Option<Vec<User>> = None;

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

fn initialize_users() {
	unsafe {
		if USERS.is_none() {
			USERS = Some(vec![
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
				),
			]);
		}
	}
}

pub fn get_users() -> &'static Vec<User> {
	unsafe {
		initialize_users();
		USERS.as_ref().unwrap()
	}
}

pub fn add_user(name: String, email: String, password: String) {
	unsafe {
		initialize_users();
		USERS.as_mut().unwrap().push(User::new(
			USERS.as_ref().unwrap().last().unwrap().id + 1,
			name,
			email,
			password,
		));
	}
}
