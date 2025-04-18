use amateraview_common::plugin::PluginHandle;
use iced::widget::pane_grid;
use std::collections::HashMap;
use uuid::Uuid;

pub struct State {
    pub panes: pane_grid::State<PluginHandle>,
    pub plugins: HashMap<PluginHandle, Plugin>,
}

impl Default for State {
    fn default() -> Self {
        let handle = PluginHandle::new();
        let plugin = Plugin {
            title: "Title".into(),
            val: 0,
        };
        let (panes, _) = pane_grid::State::new(handle);
        let mut plugins = HashMap::new();
        plugins.insert(handle, plugin);
        Self { panes, plugins }
    }
}

pub struct Plugin {
    pub title: String,
    pub val: u32,
}
