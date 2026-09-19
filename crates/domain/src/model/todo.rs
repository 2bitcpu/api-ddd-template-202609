use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TodoModel {
    pub id: String,
    pub owner: String,
    pub due: DateTime<Utc>,
    pub done: bool,
    pub title: String,
    pub note: String,
}
