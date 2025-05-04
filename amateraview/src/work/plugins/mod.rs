use crate::Message;
use eyre::WrapErr;
use std::net::{Ipv6Addr, SocketAddrV6};
use tokio::net::TcpListener;
use tokio::sync::mpsc::Sender;
use tracing::info;

pub async fn tcp_listener(_sender: Sender<Message>, port: u16) {
    let tcp = TcpListener::bind(SocketAddrV6::new(Ipv6Addr::LOCALHOST, port, 0, 0))
        .await
        .wrap_err("Could not bind to port 11_111.");

    if let Ok(tcp) = tcp {
        while let Ok((_stream, addr)) = tcp.accept().await {
            info!("New connection from {:?}", addr);
        }
    }
}

// #[derive(Debug, Clone)]
// pub struct PluginConnection {
//     pub stream: Arc<TcpStream>,
//     pub addr: SocketAddr,
// }
//
// impl PluginConnection {
//     pub fn new(stream: TcpStream, addr: SocketAddr) -> Self {
//         Self {
//             stream: Arc::new(stream),
//             addr,
//         }
//     }
// }
