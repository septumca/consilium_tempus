use axum::{extract::Path, Json};
use serde::{Serialize, Deserialize};
use chrono::prelude::*;
use mongodb::{bson::{doc, Uuid, Document}, options::{UpdateModifications}};
use futures::stream::{StreamExt};

use crate::{utils::{WithCollectionName, get_collection, PrError}, users::{User, UserPublicData}};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
  _id: Uuid,
  name: String,
  description: Option<String>,
  status: i32,
  creator: UserPublicData,
  created_on: DateTime<Utc>,
  assignee: Option<UserPublicData>,
}

impl WithCollectionName for Task {
  fn get_name() -> String {
      String::from("tasks")
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateTaskData {
  name: String,
  description: Option<String>,
  creator: UserPublicData,
  assignee: Option<UserPublicData>,
}

impl From<CreateTaskData> for Task {
  fn from(td: CreateTaskData) -> Self {
    Task {
      _id: Uuid::new(),
      name: td.name,
      description: td.description,
      creator: td.creator,
      status: 0,
      created_on: Utc::now(),
      assignee: td.assignee,
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateTaskData {
  name: Option<String>,
  description: Option<String>,
  assignee: Option<User>,
  status: Option<i32>,
}


impl From<UpdateTaskData> for UpdateModifications {
  fn from(utd: UpdateTaskData) -> Self {
    let mut doc = Document::new();
    if let Some(n) = utd.name {
      doc.insert("name", n);
    }
    if let Some(d) = utd.description {
      doc.insert("description", d);
    }
    if let Some(a) = utd.assignee {
      doc.insert("assignee", a);
    }
    if let Some(s) = utd.status {
      doc.insert("status", s);
    }
    UpdateModifications::Document(doc! { "$set": doc})
  }
}

pub async fn read_users_tasks(Path(id): Path<String>) -> Result<Json<Vec<Task>>, PrError> {
  let coll = get_collection::<Task>().await?;
  let id = Uuid::parse_str(id)?;
  let filter = doc! {
    "$or": [
      { "creator._id": id },
      { "$and": [
        { "assignee": { "$exists": true }},
        { "assignee._id": id }
      ]}
    ]
  };
  let cur = coll.find(filter, None).await?;
  let entries: mongodb::error::Result<Vec<Task>> = cur.collect::<Vec<mongodb::error::Result<Task>>>().await.into_iter().collect();

  match entries {
    Ok(e) => Ok::<Json<Vec<Task>>, PrError>(Json(e)),
    Err(e) => Err::<Json<Vec<Task>>, PrError>(PrError::from(e))
  }
}