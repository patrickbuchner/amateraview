use crate::Message;
use eyre::WrapErr;
use std::net::{Ipv6Addr, SocketAddrV6};
use tokio::net::TcpListener;
use tokio::select;
use tokio::sync::mpsc::Sender;
use tokio_util::sync::CancellationToken;
use tracing::info;

pub async fn tcp_listener(
    sender: Sender<Message>,
    port: u16,
    cancellation_token: CancellationToken,
) {
    let tcp = TcpListener::bind(SocketAddrV6::new(Ipv6Addr::LOCALHOST, port, 0, 0))
        .await
        .wrap_err(format!("TCP listener: Could not bind to port {port}."));

    match tcp {
        Ok(tcp) => loop {
            select! {
                _ = cancellation_token.cancelled() => {
                    info!("TCP listener cancelled");
                    break; }
                Ok((_stream,addr)) = tcp.accept() => {
                    info!("New connection from {:?}", addr);
                }
            }
        },
        Err(err) => {
            sender.send(Message::Error(err.to_string())).await.unwrap();
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
