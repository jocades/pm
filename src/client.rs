use crate::cmd::{Ping, Start};
use crate::message::{Message, Response};
use crate::{Command, Connection};

use log::info;
use tokio::net::{TcpStream, ToSocketAddrs};

pub struct Client {
    conn: Connection,
}

// const x: i32 = "hello";
//

impl Client {
    pub async fn connect<T: ToSocketAddrs>(addr: T) -> crate::Result<Client> {
        println!("Connecting...");
        let stream = TcpStream::connect(addr).await?;
        let conn = Connection::new(stream);
        Ok(Client { conn })
    }

    pub async fn ping(&mut self) -> crate::Result<()> {
        let cmd = Command::from(Ping::new(Some("hello")));
        self.conn.write(cmd).await?;

        let result = self.conn.read().await?;

        println!("{result:?}");

        let Some(Message::Response(res)) = result else {
            println!("Client closed connection");
            return Ok(());
        };

        println!("Reponse: {:?}", res);

        Ok(())
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
