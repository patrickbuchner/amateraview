use iced::widget::pane_grid;
use crate::state::State;

#[derive(Debug, Clone)]
pub enum PaneMessage {
    Clicked(pane_grid::Pane),
    Dragged(pane_grid::DragEvent),
    Resized(pane_grid::ResizeEvent),
}

pub fn handle_pane_changes(state: &mut State, pm: PaneMessage) {
    match pm {
        PaneMessage::Clicked(pane) => {
            state.focus = Some(pane);
        }
        PaneMessage::Dragged(pane_grid::DragEvent::Dropped { pane, target }) => {
            state.panes.drop(pane, target);
        }
        PaneMessage::Dragged(_) => {}
        PaneMessage::Resized(pane_grid::ResizeEvent { split, ratio }) => {
            state.panes.resize(split, ratio);
        }
    }
}