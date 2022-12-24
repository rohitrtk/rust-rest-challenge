use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ActorListEntry {
  pub id: i32,
  pub first_name: String,
  pub last_name: String,
  pub age: i32,
  pub dob: String,
  pub movies: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AddActorEntry {
  pub first_name: String,
  pub last_name: String,
  pub age: i32,
  pub dob: String,
  pub movies: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UpdateActorEntry {
  pub id: i32,
  pub first_name: Option<String>,
  pub last_name: Option<String>,
  pub age: Option<i32>,
  pub dob: Option<String>,
  pub movies: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ActorId {
  pub id: i32,
}