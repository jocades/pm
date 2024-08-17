use crate::Message;

use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufStream};
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

    pub async fn read_message<T>(&mut self) -> crate::Result<Option<T>>
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
            0 => {
                info!("Client closed connection");
                Ok(None)
            } // client closed the connection;
            _ => {
                let msg = String::from_utf8_lossy(&self.buffer);
                println!("Received: {msg}");

                Ok(serde_json::from_slice(&self.buffer)?)

                // self.stream
                //     .write_all(format!("Echo: {msg}").as_bytes())
                //     .await
                //     .unwrap();
                // self.stream.flush().await.unwrap();
            }
        }
    }

    pub async fn write_message<T>(&mut self, msg: T) -> crate::Result<()>
    where
        T: Serialize,
    {
        self.stream.write_all(&serde_json::to_vec(&msg)?).await?;
        self.stream.write_all(b"\n").await?;
        self.stream.flush().await?;
        Ok(())
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
