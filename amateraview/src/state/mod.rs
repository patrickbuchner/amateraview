use amateraview_common::plugin::PluginHandle;
use iced::widget::pane_grid;
use iced::widget::pane_grid::Axis;
use plugin::Plugin;
use std::collections::HashMap;
use iced::Theme;

mod plugin;

pub struct State {
    pub panes: pane_grid::State<PluginHandle>,
    pub plugins: HashMap<PluginHandle, Plugin>,
    pub focus: Option<pane_grid::Pane>,
    pub theme: Theme,
}

impl Default for State {
    fn default() -> Self {
        let (handle, plugin) = Plugin::create_demo();
        let (handle2, plugin2) = Plugin::create_demo();
        let (mut panes, pane) = pane_grid::State::new(handle);
        panes.split(Axis::Vertical, pane, handle2);
        let mut plugins = HashMap::new();
        plugins.insert(handle, plugin);
        plugins.insert(handle2, plugin2);
        Self {
            panes,
            plugins,
            focus: None,
            theme: Theme::CatppuccinMocha,
        }
    }
}

impl State {
    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
