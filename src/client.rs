use crate::cmd::{Ping, Start};
use crate::message::{Message, Response};
use crate::{Command, Connection};

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

    pub async fn ping(&mut self, msg: Option<String>) -> crate::Result<()> {
        let cmd = Command::from(Ping::new(msg));
        self.conn.write(cmd).await?;

        // self.read_response().await.map()
        match self.read_response().await? {
            Response::Ok(msg) => println!("Ok: {msg}"),
            Response::Error(msg) => println!("Error: {msg}"),
        };

        Ok(())
    }

    pub async fn start(
        &mut self,
        process: String,
        name: Option<String>,
    ) -> crate::Result<()> {
        let cmd = Command::from(Start::new(process, name));
        self.conn.write(cmd).await?;

        match self.read_response().await? {
            Response::Ok(msg) => println!("Ok: {msg}"),
            Response::Error(msg) => println!("Error: {msg}"),
        };

        Ok(())
    }

    pub async fn read_response(&mut self) -> crate::Result<Response> {
        match self.conn.read().await? {
            Some(Message::Response(res)) => Ok(res),
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
            _ => {
                // Receiving a message other than a response is unexpected.
                let err = Error::new(
                    ErrorKind::InvalidData,
                    "unexpected message from server",
                );

                Err(err.into())
            }
        }
    }
}
