use crate::Message;
use crate::state::State;
use crate::ui::PaneMessage;
use iced::Fill;
use iced::widget::{PaneGrid, container, pane_grid, responsive, row, text};
pub mod pane_type;

pub fn pane_view(state: &State) -> PaneGrid<Message> {
    let pane_grid = PaneGrid::new(&state.panes, |_id, handle, _is_maximized| {
        let plugin = state.plugins.get(handle).unwrap();
        pane_grid::Content::new(responsive(move |_a| {
            container(plugin.view()).padding(10).into()
        }))
        .style(crate::main_style)
        .title_bar(
            pane_grid::TitleBar::new(row![text(plugin.title())])
                .padding(10)
                .style(crate::main_style),
        )
    })
    .width(Fill)
    .height(Fill)
    .spacing(10)
    .on_click(|p| Message::Pane(PaneMessage::Clicked(p)))
    .on_drag(|d| Message::Pane(PaneMessage::Dragged(d)))
    .on_resize(10, |r| Message::Pane(PaneMessage::Resized(r)));
    pane_grid
}
