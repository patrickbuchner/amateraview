use crate::ui::pane::pane_type::PaneType;
use crate::work::Job;
use amateraview_common::plugin::PluginHandle;
use eyre::Result;
use iced::Theme;
use iced::widget::pane_grid;
use iced::widget::pane_grid::Axis;
use plugin::Plugin;
use std::collections::HashMap;
use tokio::sync::mpsc::Sender;
use tokio_util::sync::CancellationToken;

pub mod plugin;

pub struct State {
    pub panes: pane_grid::State<PluginHandle>,
    pub plugins: HashMap<PluginHandle, PaneType>,
    pub focus: Option<pane_grid::Pane>,
    pub theme: Theme,
    pub job_requester: Option<Sender<(Job, CancellationToken)>>,
    pub cancellation_token: CancellationToken,
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
        plugins.insert(handle, PaneType::External(plugin));
        plugins.insert(handle2, PaneType::External(plugin2));

        let token = CancellationToken::new();
        Ok(Self {
            panes,
            plugins,
            focus: None,
            theme: Theme::CatppuccinMocha,
            job_requester: None,
            cancellation_token: token,
        })
    }

    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
