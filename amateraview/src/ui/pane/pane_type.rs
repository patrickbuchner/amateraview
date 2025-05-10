use crate::Message;
use crate::state::plugin::Plugin;
use iced::Element;

pub enum PaneType {
    External(Plugin),
    Internal,
}

impl PaneType {
    pub fn view(&self) -> Element<Message> {
        match self {
            PaneType::External(e) => e.view(),
            PaneType::Internal => {
                todo!("No internal plugins defined yet.")
            }
        }
    }

    pub fn title(&self) -> String {
        match self {
            PaneType::External(e) => e.title.clone(),
            PaneType::Internal => {
                todo!("No internal plugins defined yet.")
            }
        }
    }
}
