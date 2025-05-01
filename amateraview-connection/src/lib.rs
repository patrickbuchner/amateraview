use semver::Version;
use std::net::SocketAddrV6;

pub struct Server {
    pub address: SocketAddrV6,
    pub version: Version,
}
