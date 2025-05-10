use amateraview_common::plugin::PluginHandle;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::net::SocketAddrV6;

mod length_delimited_receiver;
mod length_delimited_sender;

pub use length_delimited_receiver::LengthDelimitedReceiver;
pub use length_delimited_sender::LengthDelimitedSender;

pub struct Server {
    pub address: SocketAddrV6,
    pub version: Version,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Lifetime {
    Initiate(Client),
    Accepted(PluginHandle),
    Rejected(RejectionReason),
    AreYouAlive(PluginHandle),
    StillAlive(PluginHandle),
    BeforeShutdown,
    Shutdown,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RejectionReason {
    VersionMismatch,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Client {
    pub required_version: semver::VersionReq,
    pub name: String,
    pub handle: Option<PluginHandle>,
}
