use serde::{Deserialize, Serialize};
use crate::ui::button::Button;

pub mod button;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum WidgetState {
    Button(Button),
}



#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WidgetHandle {
    id: uuid::Uuid,
}
