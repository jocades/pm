use crate::Message;

use log::{debug, error};
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufStream};
use tokio::net::TcpStream;

pub struct Connection {
    stream: BufStream<TcpStream>,
    buffer: Vec<u8>,
}

// const x: i32 = "hello";

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        Connection {
            stream: BufStream::new(stream),
            buffer: vec![], // 1kb buffer for read operations
        }
    }

    pub async fn read(&mut self) -> crate::Result<Option<Message>> {
        match self.stream.read_until(b'\n', &mut self.buffer).await? {
            0 if !self.buffer.is_empty() => {
                // The remote closed the connection. For this to be a clean
                // shutdown, there should be no data in the read buffer. If
                // there is, this means that the peer closed the socket while
                // sending a frame.
                Err("connection closed by peer".into())
            }
            0 => Ok(None), // The remote closed the connection.
            _ => {
                debug!("Received (bytes): {:?}", self.buffer);
                debug!(
                    "Received (raw): {:?}",
                    String::from_utf8_lossy(&self.buffer)
                );

                Message::from_bytes(&self.buffer)
                    .map(|msg| {
                        debug!("Received (parsed): {:?}", msg);
                        Some(msg)
                    })
                    .map_err(|e| {
                        error!("Failed to parse message: {:?}", e);
                        e.into()
                    })
            }
        }
    }

    pub async fn write<T>(&mut self, msg: T) -> crate::Result<()>
    where
        T: Into<Message>,
    {
        let msg: Message = msg.into();
        debug!("Sent: {:?}", msg);
        self.stream.write_all(&msg.to_bytes()?).await?;
        self.stream.flush().await?;
        Ok(())
    }
}
