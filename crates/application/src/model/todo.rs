use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::model::custom_deserializers::trim_string;
use domain::model::TodoModel;

#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TodoEntryRequestDto {
    pub due: DateTime<Utc>,
    #[serde(default)]
    pub done: bool,
    #[serde(deserialize_with = "trim_string")]
    #[validate(length(min = 1, max = 300))]
    pub title: String,
    #[serde(deserialize_with = "trim_string")]
    #[validate(length(min = 1, max = 3000))]
    pub note: String,
}

impl TodoEntryRequestDto {
    pub fn to_model(&self, owner: String) -> TodoModel {
        TodoModel {
            id: uuid25::gen_v4().to_string(),
            owner,
            due: self.due,
            done: self.done,
            title: self.title.clone(),
            note: self.note.clone(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TodoReplacceRequestDto {
    #[serde(deserialize_with = "trim_string")]
    #[validate(length(min = 1, max = 32))]
    pub id: String,
    pub due: DateTime<Utc>,
    #[serde(default)]
    pub done: bool,
    #[serde(deserialize_with = "trim_string")]
    #[validate(length(min = 1, max = 300))]
    pub title: String,
    #[serde(deserialize_with = "trim_string")]
    #[validate(length(min = 1, max = 3000))]
    pub note: String,
}

impl TodoReplacceRequestDto {
    pub fn to_model(&self, owner: String) -> TodoModel {
        TodoModel {
            id: self.id.clone(),
            owner,
            due: self.due,
            done: self.done,
            title: self.title.clone(),
            note: self.note.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TodoResponseDto {
    pub id: String,
    pub due: DateTime<Utc>,
    pub done: bool,
    pub title: String,
    pub note: String,
}

impl From<TodoModel> for TodoResponseDto {
    fn from(model: TodoModel) -> Self {
        Self {
            id: model.id,
            due: model.due,
            done: model.done,
            title: model.title,
            note: model.note,
        }
    }
}
