use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, Hash, Serialize, Deserialize, PartialEq)]
pub struct PluginHandle {
    id: uuid::Uuid,
}

impl Display for PluginHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Default for PluginHandle {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginHandle {
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
        }
    }
}
