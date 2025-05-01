use amateraview_connection::Server;
use eyre::Result;
use semver::Version;
use std::net::{Ipv6Addr, SocketAddrV6};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<()> {
    let server = Server {
        address: SocketAddrV6::new(Ipv6Addr::LOCALHOST, 11_111, 0, 0),
        version: Version::new(0, 1, 0),
    };

    let socket = TcpStream::connect(server.address).await?;    
    
    

    Ok(())
}
