use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use domain::model::TodoModel;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TodoData {
    pub map: HashMap<String, TodoModel>,
    pub dirty: bool,
}

impl TodoData {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            dirty: false,
        }
    }
}
