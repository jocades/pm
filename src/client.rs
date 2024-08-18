use crate::cmd::{Ping, Start};
use crate::{Command, Connection};

use serde_json::Value;
use std::io::{Error, ErrorKind};
use tokio::net::{TcpStream, ToSocketAddrs};

pub struct Client {
    pub conn: Connection,
}

impl Client {
    pub async fn connect<T: ToSocketAddrs>(addr: T) -> crate::Result<Client> {
        println!("Connecting...");
        let stream = TcpStream::connect(addr).await?;
        let conn = Connection::new(stream);
        Ok(Client { conn })
    }

    pub async fn ping(&mut self, msg: Option<&str>) -> crate::Result<()> {
        let cmd = Command::from(Ping::new(msg));
        self.conn.write_message(&cmd).await?;

        let response = self.read_response().await?;
        println!("{}", response["data"]);

        Ok(())
    }

    pub async fn start(
        &mut self,
        process: &str,
        name: Option<&str>,
    ) -> crate::Result<()> {
        let cmd = Command::from(Start::new(process, name));
        self.conn.write_message(&cmd).await?;

        let response = self.read_response().await?;
        println!("{}", response);

        Ok(())
    }

    pub async fn read_response(&mut self) -> crate::Result<Value> {
        match self.conn.read_message().await? {
            Some(msg) => Ok(msg),
            None => {
                // Receiving `None` here indicates the server has closed the
                // connection without sending a response. This is unexpected
                // and is represented as a "connection reset by peer" error.
                let err = Error::new(
                    ErrorKind::ConnectionReset,
                    "connection reset by server",
                );

                Err(err.into())
            }
        }
    }
}
