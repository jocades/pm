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

    pub async fn ping(&mut self) -> crate::Result<Response> {
        let cmd = Command::from(Ping::new(Some("hello")));
        self.conn.write(cmd).await?;

        self.read_response().await

        // match res {
        //     Response::Ok(val) => {
        //         println!("{val}")
        //     }
        //     Response::Error(val) => {
        //         println!("{val}")
        //     }
        // }

        // println!("Reponse: {:?}", res);
        //
        // Ok(())
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

    /* pub async fn start(
        &mut self,
        process: &str,
        name: Option<&str>,
    ) -> crate::Result<()> {
        // let cmd = Start::new(process, name);
        let cmd = Start::new(process, name);
        self.conn.send(cmd).await?;
    } */
}
