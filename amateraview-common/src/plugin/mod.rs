use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, Hash, Serialize, Deserialize, PartialEq)]
pub struct PluginHandle {
    id: uuid::Uuid,
}

impl PluginHandle {
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
        }
    }
}
