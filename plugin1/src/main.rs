use amateraview_common::communication::MessageFromPlugin;
use amateraview_connection::LengthDelimitedReceiver;
use amateraview_connection::LengthDelimitedSender;
use amateraview_connection::{Client, Lifetime, Server};
use eyre::{Result, WrapErr};
use semver::{Version, VersionReq};
use std::net::{Ipv6Addr, SocketAddrV6};
use tokio::net::TcpStream;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .pretty()
        // Display source code file paths
        .with_file(true)
        // Display source code line numbers
        .with_line_number(true)
        // Display the thread ID an event was recorded on
        .with_thread_ids(true)
        // Don't display the event's target (module path)
        .with_target(true)
        // Build the subscriber
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .wrap_err("Failed to initialize the subscriber.")?;

    let server = Server {
        address: SocketAddrV6::new(Ipv6Addr::LOCALHOST, 11_111, 0, 0),
        version: Version::new(0, 1, 0),
    };

    let socket = TcpStream::connect(server.address).await?;
    _ = tokio::spawn(make_handshake(socket)).await;
    Ok(())
}

async fn make_handshake(stream: TcpStream) {
    let (reader, writer) = tokio::io::split(stream);

    let mut sender = LengthDelimitedSender::new(writer);
    let mut reader = LengthDelimitedReceiver::new(reader);

    let intro = Client {
        required_version: VersionReq::parse(">=0.2.0").unwrap(),
        name: "Plugin1".into(),
        handle: None,
    };
    sender.send(&Lifetime::Initiate(intro)).await.unwrap();

    let answer = reader.receive::<Lifetime>().await.unwrap();
    info!("{:?}", answer);
}
