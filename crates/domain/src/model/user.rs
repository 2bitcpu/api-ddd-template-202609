use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserModel {
    pub account: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub password: String,
    pub jwt_id: Option<String>,
}
