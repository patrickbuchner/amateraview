use crate::state::State;
use amateraview_common::plugin::PluginHandle;
use amateraview_common::ui::WidgetHandle;
use eyre::{Context, Report, Result};
use iced::futures::SinkExt;
use iced::widget::container::Style;
use iced::widget::{PaneGrid, container, pane_grid, pick_list, responsive, row, text};
use iced::{Border, Element, Fill, Length, Subscription, Theme, stream};
use std::net::{Ipv6Addr, SocketAddr, SocketAddrV6};
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tracing::{error, info};

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
        .theme(State::theme)
        .centered()
        .subscription(tcp_listener)
        .run()
        .wrap_err("Failed to run the application.")
}

#[derive(Debug, Clone)]
enum Message {
    ButtonPressed(PluginHandle, WidgetHandle),
    Clicked(pane_grid::Pane),
    Dragged(pane_grid::DragEvent),
    Resized(pane_grid::ResizeEvent),
    ThemeChanged(Theme),
    UnhandledError(Arc<Report>),
    ReceivedConnectionRequest(PluginConnection),
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::ButtonPressed(handle, widget) => {
            let plugin = state.plugins.get_mut(&handle).unwrap();
            plugin.triggered(widget);
        }
        Message::Clicked(pane) => {
            state.focus = Some(pane);
        }
        Message::Dragged(pane_grid::DragEvent::Dropped { pane, target }) => {
            state.panes.drop(pane, target);
        }
        Message::Dragged(_) => {}
        Message::Resized(pane_grid::ResizeEvent { split, ratio }) => {
            state.panes.resize(split, ratio);
        }
        Message::ThemeChanged(theme) => state.theme = theme,

        Message::ReceivedConnectionRequest(plugin_connection) => {
            info!(
                "A possible plugin connected from {:?}",
                plugin_connection.addr
            );
        }
        Message::UnhandledError(e) => error!("{e}"),
    }
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
    .on_click(Message::Clicked)
    .on_drag(Message::Dragged)
    .on_resize(10, Message::Resized);

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

fn tcp_listener(state: &State) -> Subscription<Message> {
    Subscription::run_with_id(
        "Tcp Listener",
        stream::channel(100, |mut output| async move {
            let tcp = TcpListener::bind(SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, 11_111, 0, 0))
                .await
                .wrap_err("Could not bind to port 11_111.");

            match tcp {
                Ok(tcp) => loop {
                    if let Ok((stream, addr)) = tcp.accept().await {
                        output
                            .send(Message::ReceivedConnectionRequest(PluginConnection::new(
                                stream, addr,
                            )))
                            .await;
                    }
                },
                Err(err) => {
                    output.send(Message::UnhandledError(Arc::new(err))).await;
                }
            }
        }),
    )
}

#[derive(Debug, Clone)]
pub struct PluginConnection {
    pub stream: Arc<TcpStream>,
    pub addr: SocketAddr,
}
impl PluginConnection {
    pub fn new(stream: TcpStream, addr: SocketAddr) -> Self {
        Self {
            stream: Arc::new(stream),
            addr,
        }
    }
}
