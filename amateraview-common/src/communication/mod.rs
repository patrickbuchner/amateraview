use crate::plugin::PluginHandle;
use crate::ui::{WidgetHandle, WidgetState};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MessageFromPlugin {
    Request(PluginHandle, WidgetState),
    Update(PluginHandle, WidgetHandle, WidgetState),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MessageToPlugin {
    Response(WidgetHandle, WidgetState),
    Update(WidgetHandle, WidgetState),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Lifetime {
    Initiate(Option<PluginHandle>),
    Accepted(PluginHandle),
    AreYouAlive(PluginHandle),
    StillAlive(PluginHandle),
    BeforeShutdown,
    Shutdown,
}
