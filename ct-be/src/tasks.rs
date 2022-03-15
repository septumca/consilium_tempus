use serde::{Serialize, Deserialize};
use chrono::prelude::*;
use mongodb::{bson::{doc, Uuid, Document}, options::{UpdateModifications}};

use crate::{utils::WithCollectionName, users::User};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
  _id: Uuid,
  name: String,
  description: Option<String>,
  status: i32,
  creator: User,
  created_on: DateTime<Utc>,
  assignee: Option<User>,
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
  creator: User,
  assignee: Option<User>,
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
