use crate::state::State;
use crate::work::Job;
use amateraview_common::plugin::PluginHandle;
use amateraview_common::ui::WidgetHandle;
use eyre::{Context, Result};
use iced::widget::container::Style;
use iced::widget::{container, pick_list, text};
use iced::window::Position;
use iced::{Border, Element, Event, Fill, Length, Point, Subscription, Task, Theme, event, window};
use tokio::sync::mpsc::Sender;
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

pub mod ui;
use crate::ui::pane::pane_type::PaneType;
use ui::{PaneMessage, pane};

pub mod configuration;

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
        .position(Position::Specific(Point { x: 0., y: 0. }))
        .subscription(|state| {
            Subscription::batch([
                work::worker_listener(state),
                event::listen().map(Message::EventOccurred),
            ])
        })
        .exit_on_close_request(false)
        .run()
        .wrap_err("Failed to run the application.")
}

#[derive(Debug, Clone)]
pub enum Message {
    ButtonPressed(PluginHandle, WidgetHandle),
    EventOccurred(Event),
    Pane(PaneMessage),
    ThemeChanged(Theme),
    MainWorkLoop(Sender<(Job, CancellationToken)>),
    Request((Job, CancellationToken)),
    Error(String),
    StartShutdown,
    Shutdown,
}

fn view(state: &State) -> Element<'_, Message> {
    let pane_grid = pane::pane_view(state);

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
            match plugin {
                PaneType::External(e) => {
                    e.triggered(widget);
                }
                PaneType::Internal => {
                    todo!("Internal panes not implemented yet.")
                }
            }
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
            Task::done(Message::Request((
                Job::ListenForPlugins { port: 11_111 },
                state.cancellation_token.clone(),
            )))
        }
        Message::Error(message) => {
            error!(message);
            Task::done(Message::StartShutdown)
        }
        Message::StartShutdown => {
            state.cancellation_token.cancel();
            Task::done(Message::Shutdown)
        }
        Message::Shutdown => iced::exit(),
        Message::EventOccurred(event) => handle_events(event),
    }
}

fn handle_events(e: Event) -> Task<Message> {
    match e {
        Event::Keyboard(_) => Task::none(),
        Event::Mouse(_) => Task::none(),
        Event::Window(we) => match we {
            window::Event::CloseRequested => Task::done(Message::StartShutdown),
            _ => Task::none(),
        },
        Event::Touch(_) => Task::none(),
    }
}
