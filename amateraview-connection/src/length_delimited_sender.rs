use serde::Serialize;
use tokio::io::{AsyncWriteExt, WriteHalf};
use tokio::net::TcpStream;

pub struct LengthDelimitedSender {
    buf: Vec<u8>,
    writer: WriteHalf<TcpStream>,
}

impl LengthDelimitedSender {
    pub fn new(writer: WriteHalf<TcpStream>) -> LengthDelimitedSender {
        LengthDelimitedSender {
            buf: Vec::new(),
            writer,
        }
    }

    pub async fn send<T>(&mut self, value: &T) -> Result<(), std::io::Error>
    where
        T: Serialize + ?Sized,
    {
        let message = rmp_serde::to_vec(value).unwrap();
        let length = message.len();
        self.buf.reserve(message.len());
        self.writer.write_all(&length.to_le_bytes()).await?;
        self.writer.write_all(message.as_slice()).await
    }
}
