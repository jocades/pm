use crate::cmd::{Ping, Start};
use crate::{Command, Connection};

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
        // let subcmd = Ping::new(Some("hello?"));
        // let cmd = Command::Ping(Ping::new(Some("hello?")));
        // if let Command::Ping(subcmd) = cmd {
        //     subcmd.execute(&mut self.conn).await?;
        // }
        // let cmd: Command = subcmd.into();
        // self.conn.write(subcmd).await?;
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
