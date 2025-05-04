use crate::state::State;
use crate::work::Jobs;
use amateraview_common::plugin::PluginHandle;
use amateraview_common::ui::WidgetHandle;
use eyre::{Context, Result};
use iced::widget::container::Style;
use iced::widget::{PaneGrid, container, pane_grid, pick_list, responsive, row, text};
use iced::{Border, Element, Fill, Length, Task, Theme};
use tokio::sync::mpsc::Sender;
use tracing::info;

pub mod ui;
use ui::PaneMessage;

mod state;
mod work;

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
        .theme(State::theme)
        .centered()
        .subscription(work::worker_listener)
        .run()
        .wrap_err("Failed to run the application.")
}

#[derive(Debug, Clone)]
pub enum Message {
    ButtonPressed(PluginHandle, WidgetHandle),
    Pane(PaneMessage),
    ThemeChanged(Theme),
    MainWorkLoop(Sender<Jobs>),
    Request(Jobs),
}

fn view(state: &State) -> Element<'_, Message> {
    let pane_grid = PaneGrid::new(&state.panes, |_id, handle, _is_maximized| {
        let plugin = state.plugins.get(handle).unwrap();
        pane_grid::Content::new(responsive(move |_a| {
            container(plugin.view()).padding(10).into()
        }))
        .style(main_style)
        .title_bar(
            pane_grid::TitleBar::new(row![text(plugin.title.clone())])
                .padding(10)
                .style(main_style),
        )
    })
    .width(Fill)
    .height(Fill)
    .spacing(10)
    .on_click(|p| Message::Pane(PaneMessage::Clicked(p)))
    .on_drag(|d| Message::Pane(PaneMessage::Dragged(d)))
    .on_resize(10, |r| Message::Pane(PaneMessage::Resized(r)));

    let choose_theme = iced::widget::row![
        text("Theme:"),
        pick_list(Theme::ALL, Some(&state.theme), Message::ThemeChanged).width(Length::Shrink),
    ]
    .spacing(5)
    .padding(10);

    let content = container(pane_grid).width(Fill).height(Fill).padding(10);

    iced::widget::column![choose_theme, content].into()
}

pub fn main_style(theme: &Theme) -> Style {
    let palette = theme.extended_palette();

    Style {
        background: Some(palette.background.base.color.into()),
        border: Border {
            width: 2.0,
            color: palette.background.strong.color,
            ..Border::default()
        },
        ..Default::default()
    }
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::ButtonPressed(handle, widget) => {
            let plugin = state.plugins.get_mut(&handle).unwrap();
            plugin.triggered(widget);
            Task::none()
        }
        Message::Pane(pm) => {
            ui::handle_pane_changes(state, pm);
            Task::none()
        }
        Message::ThemeChanged(theme) => {
            state.theme = theme;
            Task::none()
        }
        Message::Request(job) => {
            if let Some(sender) = &state.job_requester {
                _ = sender.blocking_send(job);
            }
            Task::none()
        }
        Message::MainWorkLoop(sender) => {
            state.job_requester = Some(sender);
            info!("Got job loop started up and request it listening for plugins on port 11_111.");
            Task::done(Message::Request(Jobs::ListenForPlugins { port: 11_111 }))
        }
    }
}
