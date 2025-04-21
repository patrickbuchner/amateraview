use crate::ui::button::Button;
use serde::{Deserialize, Serialize};

pub mod button;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum WidgetState {
    Button(Button),
    Text(String),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TreeView {
    Leaf(WidgetHandle),
    Row(Vec<TreeView>),
    Column(Vec<TreeView>),
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct WidgetHandle {
    id: uuid::Uuid,
}

impl std::fmt::Display for WidgetHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Default for WidgetHandle {
    fn default() -> Self {
        Self::new()
    }
}

impl WidgetHandle {
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
        }
    }
}
