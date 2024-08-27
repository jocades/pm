use crate::cmd::{List, Ping, Start, Stop};
use crate::server::Response;
use crate::state::Task;
use crate::{Command, Connection};

use std::io::{Error, ErrorKind};
use tokio::net::{TcpStream, ToSocketAddrs};

pub struct Client {
    pub conn: Connection,
}

impl Client {
    pub async fn connect<T: ToSocketAddrs>(addr: T) -> crate::Result<Client> {
        let stream = TcpStream::connect(addr).await?;
        let conn = Connection::new(stream);
        Ok(Client { conn })
    }

    pub async fn ping(&mut self, msg: Option<&str>) -> crate::Result<()> {
        let cmd = Command::from(Ping::new(msg));
        self.conn.write(&cmd).await?;

        let response = self.read_response::<String>().await?;
        println!("{:?}", response);

        Ok(())
    }

    pub async fn start(
        &mut self,
        task: &str,
        name: Option<&str>,
    ) -> crate::Result<()> {
        let cmd = Command::from(Start::new(task, name));
        self.conn.write(&cmd).await?;

        let res = self.read_response::<Task>().await?;
        println!("{:?}", res);

        Ok(())
    }

    pub async fn stop(&mut self, name: &str) -> crate::Result<()> {
        let cmd = Command::from(Stop::new(name));
        self.conn.write(&cmd).await?;

        let res = self.read_response::<String>().await?;
        println!("{:?}", res);

        Ok(())
    }

    pub async fn list(&mut self) -> crate::Result<Vec<Task>> {
        let cmd = Command::from(List);
        self.conn.write(&cmd).await?;

        let res = self.read_response::<Vec<Task>>().await?;
        Ok(res.data)
    }

    pub async fn read_response<T>(&mut self) -> crate::Result<Response<T>>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        /* match self.conn.read().await? {
            Some(res) => Ok(res),
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
        } */

        self.conn.read().await?.ok_or_else(|| {
            let err = Error::new(
                ErrorKind::ConnectionReset,
                "connection reset by server",
            );

            err.into()
        })
    }
}
