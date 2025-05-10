use serde::de::DeserializeOwned;
use tokio::io::{AsyncReadExt, ReadHalf};
use tokio::net::TcpStream;

pub struct LengthDelimitedReceiver {
    buf: Vec<u8>,
    len_buf: [u8; size_of::<usize>()],
    reader: ReadHalf<TcpStream>,
}

impl LengthDelimitedReceiver {
    pub fn new(reader: ReadHalf<TcpStream>) -> LengthDelimitedReceiver {
        LengthDelimitedReceiver {
            buf: Vec::new(),
            len_buf: [0; size_of::<usize>()],
            reader,
        }
    }

    pub async fn receive<T>(&mut self) -> Result<T, rmp_serde::decode::Error>
    where
        T: DeserializeOwned,
    {
        self.reader.read_exact(&mut self.len_buf).await.unwrap();
        let length = usize::from_le_bytes(self.len_buf);
        self.buf.resize(length, 0);
        self.reader.read_exact(&mut self.buf).await.unwrap();
        rmp_serde::from_slice::<T>(&self.buf)
    }
}
