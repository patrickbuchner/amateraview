use amateraview_common::plugin::PluginHandle;
use eyre::{Context, Result};
use iced::Theme;
use iced::widget::pane_grid;
use iced::widget::pane_grid::Axis;
use plugin::Plugin;
use std::collections::HashMap;
use std::net::{Ipv6Addr, SocketAddrV6};
use tokio::net::TcpListener;

mod plugin;

pub struct State {
    pub panes: pane_grid::State<PluginHandle>,
    pub plugins: HashMap<PluginHandle, Plugin>,
    pub focus: Option<pane_grid::Pane>,
    pub theme: Theme,
}

impl Default for State {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

impl State {
    pub fn new() -> Result<Self> {
        let (handle, plugin) = Plugin::create_demo();
        let (handle2, plugin2) = Plugin::create_demo();
        let (mut panes, pane) = pane_grid::State::new(handle);
        panes.split(Axis::Vertical, pane, handle2);
        let mut plugins = HashMap::new();
        plugins.insert(handle, plugin);
        plugins.insert(handle2, plugin2);

        Ok(Self {
            panes,
            plugins,
            focus: None,
            theme: Theme::CatppuccinMocha,
        })
    }

    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
