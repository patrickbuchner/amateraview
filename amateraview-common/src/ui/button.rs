use serde::{Deserialize, Serialize};

#[derive( Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Button {
    pub state: ButtonState,
    pub description: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ButtonState {
    Pressed,
    Released,
}