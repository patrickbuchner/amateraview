use crate::state::State;
use amateraview_common::plugin::PluginHandle;
use amateraview_common::ui::WidgetHandle;
use eyre::{Context, Result};
use iced::widget::container::Style;
use iced::widget::{
     PaneGrid, container, pane_grid, responsive, row,
    text,
};
use iced::{Border, Element, Fill, Theme};
use tracing::{info, instrument};

mod state;

fn main() -> Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .pretty()
        // Display source code file paths
        .with_file(true)
        // Display source code line numbers
        .with_line_number(true)
        // Display the thread ID an event was recorded on
        .with_thread_ids(true)
        // Don't display the event's target (module path)
        .with_target(true)
        // Build the subscriber
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .wrap_err("Failed to initialize the subscriber.")?;

    iced::application("A counter", update, view)
        // .theme(|_| Theme::Dark)
        .centered()
        .run()
        .wrap_err("Failed to run the application.")
}

#[derive(Debug, Clone)]
enum Message {
    ButtonPressed(PluginHandle, WidgetHandle),
}
#[instrument(skip(state))]
fn update(state: &mut State, message: Message) {
    info!("{:?}", message);
    if let Message::ButtonPressed(handle, widget) = message {
        let plugin = state.plugins.get_mut(&handle).unwrap();
        plugin.triggered(widget);
    }
    info!("Leaving");
}

fn view(state: &State) -> Element<'_, Message> {
    let pane_grid = PaneGrid::new(&state.panes, |_id, handle, _is_maximized| {
        let plugin = state.plugins.get(handle).unwrap();
        pane_grid::Content::new(responsive(move |_a| {
            container(plugin.view()).padding(10).into()
        }))
        .title_bar(
            pane_grid::TitleBar::new(row![text(plugin.title.clone())])
                .padding(10)
                .style(title_bar_style),
        )
    })
    .width(Fill)
    .height(Fill)
    .spacing(10);

    container(pane_grid)
        .width(Fill)
        .height(Fill)
        .padding(10)
        .into()
}

fn title_bar_style(theme: &Theme) -> Style {
    let palette = theme.extended_palette();

    container::Style {
        background: Some(palette.background.weak.color.into()),
        border: Border {
            width: 2.0,
            color: palette.background.strong.color,
            ..Border::default()
        },
        ..Default::default()
    }
}
