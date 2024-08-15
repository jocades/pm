use crate::cmd::Executor;
use crate::message::{Message, Response};
use crate::{Connection, Error, LOCAL_HOST};

use log::{error, info, warn};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};

type Db = Arc<Mutex<HashMap<String, u32>>>;

pub async fn run(port: u16) -> crate::Result<()> {
    let listener = TcpListener::bind(format!("{LOCAL_HOST}:{port}")).await?;

    info!("Listening on port {port}");

    let db: Db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (stream, addr) = listener.accept().await?;
        let db = db.clone();

        info!("Accepted {addr}");
        tokio::spawn(async move {
            if let Err(e) = handle(stream, db).await {
                error!("Connection error: {e}");
            }
        });
    }
}

async fn handle(stream: TcpStream, _db: Db) -> crate::Result<()> {
    let mut conn = Connection::new(stream);

    match conn.read().await {
        Ok(msg) => match msg {
            None => Ok(()),
            Some(Message::Request(cmd)) => cmd.execute(&mut conn).await,
            Some(msg) => {
                let emsg = "protocol error; unexpected message";
                let res = Response::Error(emsg.to_string());
                conn.write(res).await?;
                Err(format!("{emsg}: {msg:?}").into())
            }
        },
        Err(e) => match e {
            Error::Parse(e) => {
                let res = Response::Error(e.to_string());
                conn.write(res).await?;
                Err(e.into())
            }
            e => {
                let res = Response::Error(format!("internal server error"));
                conn.write(res).await?;
                Err(e)
            }
        },
    }
}
