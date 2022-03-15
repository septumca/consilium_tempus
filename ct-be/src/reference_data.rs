
use axum::Json;
use mongodb::bson::{doc, Uuid};
use serde::{Serialize, Deserialize};

use crate::utils::{WithCollectionName, PrError, get_collection};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Status {
  name: String,
  code: i32
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReferenceData {
  _id: Uuid,
  statuses: Vec<Status>
}

impl WithCollectionName for ReferenceData {
  fn get_name() -> String {
      String::from("reference_data")
  }
}

pub async fn read() -> Result<Json<ReferenceData>, PrError> {
  let coll = get_collection::<ReferenceData>().await?;
  let entry = coll.find_one(None, None).await?;

  match entry {
    Some(e) => Ok::<Json<ReferenceData>, PrError>(Json(e)),
    None => Err::<Json<ReferenceData>, PrError>(PrError::NotFound(String::from("Reference data not found")))
  }
}