use axum::Json;
use serde::{Serialize, Deserialize};
use mongodb::{bson::{doc, Uuid, Bson}, options::{UpdateModifications}};

use crate::{utils::{WithCollectionName, PrError, get_collection}, auth::{generate_salt, get_salted_password, generate_jwt}};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
  _id: Uuid,
  name: String,
  password: String,
  salt: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserPublicData {
  _id: Uuid,
  name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserAuthReqData {
  name: String,
  password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserAuthRespData {
  _id: Uuid,
  name: String,
  token: String,
}

impl WithCollectionName for User {
  fn get_name() -> String {
      String::from("users")
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateUserData {
  name: String,
  password: String
}

impl From<CreateUserData> for User {
  fn from(ud: CreateUserData) -> Self {
    let salt = generate_salt();
    let password = get_salted_password(ud.password, salt.clone());
    Self {
      _id: Uuid::new(),
      name: ud.name,
      password,
      salt,
    }
  }
}

impl From<User> for UserPublicData {
  fn from(ud: User) -> Self {
    Self {
      _id: ud._id,
      name: ud.name,
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateUserData {
  name: String,
}

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

pub async fn create(Json(data): Json<CreateUserData>) -> Result<Json<UserPublicData>, PrError> {
  let coll = get_collection::<User>().await?;

  let entry = coll.find_one(doc! { "name": data.name.clone() }, None).await?;

  if entry.is_some() {
    return Err(PrError::Conflict(String::from("User already exists")));
  }

  let entry = User::from(data);
  coll.insert_one(entry.clone(), None).await?;

  Ok::<Json<UserPublicData>, PrError>(Json(UserPublicData::from(entry)))
}

pub async fn authentificate(Json(data): Json<UserAuthReqData>) -> Result<Json<UserAuthRespData>, PrError> {
  let coll = get_collection::<User>().await?;

  let entry = coll
    .find_one(doc! { "name": data.name }, None)
    .await?
    .ok_or_else(|| PrError::NotFound(String::from("entry not found")))?;

  let calculated_password = get_salted_password(data.password, entry.salt);

  if calculated_password != entry.password {
    return Err(PrError::Unauthorized(String::from("incorrect password")));
  }

  let resp = UserAuthRespData {
    _id: entry._id,
    name: entry.name,
    token: generate_jwt(entry._id.to_string())
  };
  Ok::<Json<UserAuthRespData>, PrError>(Json(resp))

}