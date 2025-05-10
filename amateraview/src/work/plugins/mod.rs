use crate::Message;
use amateraview_common::plugin::PluginHandle;
use amateraview_connection::{
    LengthDelimitedReceiver, LengthDelimitedSender, Lifetime, RejectionReason,
};
use eyre::WrapErr;
use semver::Version;
use std::net::{Ipv6Addr, SocketAddr, SocketAddrV6};
use tokio::net::{TcpListener, TcpStream};
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
                Ok((stream, addr)) = tcp.accept() => {
                    tokio::spawn(make_handshake(stream, addr));
                }
            }
        },
        Err(err) => {
            sender.send(Message::Error(err.to_string())).await.unwrap();
        }
    }
}

pub async fn make_handshake(stream: TcpStream, addr: SocketAddr) {
    info!("New connection from {:?}", addr);
    let (reader, writer) = tokio::io::split(stream);
    let mut sender = LengthDelimitedSender::new(writer);
    let mut reader = LengthDelimitedReceiver::new(reader);

    if let Ok(Lifetime::Initiate(plugin_introduction)) = reader.receive::<Lifetime>().await {
        info!("Plugin introduction: {:?}", plugin_introduction);
        if plugin_introduction
            .required_version
            .matches(&Version::new(0, 1, 0))
        {
            let handle = PluginHandle::new();
            sender.send(&Lifetime::Accepted(handle)).await.unwrap();
        } else {
            sender
                .send(&Lifetime::Rejected(RejectionReason::VersionMismatch))
                .await
                .unwrap();
        }
    }
}
