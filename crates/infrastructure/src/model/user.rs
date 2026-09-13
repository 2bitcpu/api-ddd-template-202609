use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use domain::model::UserModel;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserData {
    pub map: HashMap<String, UserModel>,
    pub dirty: bool,
}

impl UserData {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            dirty: false,
        }
    }
}
