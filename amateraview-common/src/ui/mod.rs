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
