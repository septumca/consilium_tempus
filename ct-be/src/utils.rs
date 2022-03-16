use mongodb::{Client, options::ClientOptions, Collection, bson::{doc, Uuid}, options::UpdateModifications};
use axum::{response::{IntoResponse, Response}, http::StatusCode, Json, extract::Path};
use serde_json::json;
use serde::{Serialize, Deserialize, de::DeserializeOwned};
use futures::stream::{StreamExt};

pub trait WithCollectionName {
  fn get_name() -> String;
}

pub enum PrError {
  DB(String),
  Server(String),
  NotFound(String)
}

impl IntoResponse for PrError {
  fn into_response(self) -> Response {
    let (status, error_message) = match self {
      PrError::Server(msg) => {
        (
          StatusCode::INTERNAL_SERVER_ERROR,
          format!("{} - {}", "Internal Server Error", msg)
        )
      }
      PrError::DB(msg) => {
        (
          StatusCode::INTERNAL_SERVER_ERROR,
          format!("{} - {}", "Internal Server Error - Database", msg)
        )
      }
      PrError::NotFound(msg) => {
        (
          StatusCode::NOT_FOUND,
          format!("{} - {}", "Not Found", msg)
        )
      }
    };
    let body = Json(json!({
      "error": error_message,
    }));

    (status, body).into_response()
  }
}

impl From<axum::Error> for PrError {
  fn from(e: axum::Error) -> Self {
      PrError::Server(e.to_string())
  }
}

impl From<mongodb::error::Error> for PrError {
  fn from(e: mongodb::error::Error) -> Self {
      PrError::DB(e.to_string())
  }
}

impl From<mongodb::bson::uuid::Error> for PrError {
  fn from(e: mongodb::bson::uuid::Error) -> Self {
    PrError::Server(e.to_string())
  }
}


pub async fn get_collection<T>() -> Result<Collection<T>, PrError>
where T: Serialize + WithCollectionName,
{
  let client_options = ClientOptions::parse("mongodb://root:example@localhost:27017").await?;
  let client = Client::with_options(client_options)?;
  Ok(client.database("consilium_tempus").collection::<T>(T::get_name().as_str()))
}


pub async fn create<'de, D, T>(Json(data): Json<D>) -> Result<Json<T>, PrError>
where D: Deserialize<'de> + std::fmt::Debug,
      T: From<D> + Serialize + Clone + WithCollectionName + std::fmt::Debug,
{
  let coll = get_collection::<T>().await?;
  let entry = T::from(data);
  coll.insert_one(entry.clone(), None).await?;

  Ok::<Json<T>, PrError>(Json(entry))
}

pub async fn read<T>(Path(id): Path<String>) -> Result<Json<T>, PrError>
where T: Serialize + DeserializeOwned + Unpin + std::marker::Send + Sync + WithCollectionName,
{
  let coll = get_collection::<T>().await?;
  let id = Uuid::parse_str(id)?;
  let entry = coll
    .find_one(doc! { "_id": id }, None)
    .await?
    .ok_or_else(|| PrError::NotFound(String::from("entry not found")))?;

  Ok::<Json<T>, PrError>(Json(entry))
}

pub async fn read_all<T>() -> Result<Json<Vec<T>>, PrError>
where T: Serialize + DeserializeOwned + Unpin + std::marker::Send + Sync + WithCollectionName,
{
  let coll = get_collection::<T>().await?;
  let cur = coll.find(None, None).await?;
  let entries: mongodb::error::Result<Vec<T>> = cur.collect::<Vec<mongodb::error::Result<T>>>().await.into_iter().collect();

  match entries {
    Ok(e) => Ok::<Json<Vec<T>>, PrError>(Json(e)),
    Err(e) => Err::<Json<Vec<T>>, PrError>(PrError::from(e))
  }
}

pub async fn put<'de, D, T>(Path(id): Path<String>, Json(data): Json<D>) -> Result<(), PrError>
where D: Serialize + Deserialize<'de> + Into<UpdateModifications>,
      T: Serialize + DeserializeOwned + Unpin + std::marker::Send + Sync + WithCollectionName,
{
  let coll = get_collection::<T>().await?;
  let id = Uuid::parse_str(id)?;
  let _ = coll.update_one(doc! { "_id": id }, data, None).await?;

  Ok::<(), PrError>(())
}


pub async fn delete<T>(Path(id): Path<String>) -> Result<(), PrError>
where T: Serialize + DeserializeOwned + Unpin + std::marker::Send + Sync + WithCollectionName,
{
  let coll = get_collection::<T>().await?;
  let id = Uuid::parse_str(id)?;
  coll.delete_one(doc! { "_id": id }, None).await?;

  Ok::<(), PrError>(())
}