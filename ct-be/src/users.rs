use serde::{Serialize, Deserialize};
use mongodb::{bson::{doc, Uuid, Bson}, options::{UpdateModifications}};

use crate::utils::WithCollectionName;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
  _id: Uuid,
  name: String,
}

impl WithCollectionName for User {
  fn get_name() -> String {
      String::from("users")
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateUserData {
  name: String
}

impl From<CreateUserData> for User {
  fn from(ud: CreateUserData) -> Self {
    Self {
      _id: Uuid::new(),
      name: ud.name,
    }
  }
}

pub type UpdateUserData = CreateUserData;

impl From<UpdateUserData> for UpdateModifications {
  fn from(d: UpdateUserData) -> Self {
    UpdateModifications::Document(doc! { "$set": { "name": d.name }})
  }
}

impl From<User> for Bson {
  fn from(d: User) -> Self {
    Bson::Document(doc! { "_id": d._id, "name": d.name, })
  }
}
