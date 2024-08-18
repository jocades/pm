use log::{debug, error};
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufStream};
use tokio::net::TcpStream;

pub struct Connection {
    stream: BufStream<TcpStream>,
    buffer: Vec<u8>,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        Connection {
            stream: BufStream::new(stream),
            buffer: Vec::with_capacity(1024),
        }
    }

    /// Only returns `None` if the remote closed the connection.
    pub async fn read<T>(&mut self) -> crate::Result<Option<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        self.buffer.clear();

        // bubble up any read error (io error)
        match self.stream.read_until(b'\n', &mut self.buffer).await? {
            0 if !self.buffer.is_empty() => {
                // The remote closed the connection. For this to be a clean
                // shutdown, there should be no data in the read buffer. If
                // there is, this means that the peer closed the socket while
                // sending a frame.
                Err("connection closed by peer".into())
            }
            0 => Ok(None), // connection closed
            _ => {
                debug!("Received (bytes): {:?}", self.buffer);
                debug!(
                    "Received (raw): {:?}",
                    String::from_utf8_lossy(&self.buffer)
                );

                // Ok(serde_json::from_slice(&self.buffer)?)

                serde_json::from_slice(&self.buffer).map_err(|e| {
                    error!("Failed to parse message: {:?}", e);
                    e.into()
                })
            }
        }
    }

    pub async fn write<T>(&mut self, msg: &T) -> crate::Result<()>
    where
        T: Serialize,
    {
        self.stream.write_all(&serde_json::to_vec(msg)?).await?;
        self.stream.write_all(b"\n").await?;
        self.stream.flush().await?;
        Ok(())
    }
}
