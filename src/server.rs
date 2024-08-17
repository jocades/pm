use crate::cmd::Executor;
use crate::db::Db;
use crate::message::{Message, Response};
use crate::{Connection, Error, LOCAL_HOST};

use log::{error, info, warn};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::{env, fs};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufStream};
use tokio::net::{TcpListener, TcpStream};

pub async fn run(port: u16) -> crate::Result<()> {
    let listener = TcpListener::bind(format!("{LOCAL_HOST}:{port}")).await?;

    info!("Server started with pid: {}", std::process::id());
    info!("PM_DIR: {:?}", env::var("PM_DIR"));
    info!("Listening on port {port}");

    let db: Db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (stream, addr) = listener.accept().await?;
        let db = db.clone();

        info!("Accepted {addr}");
        tokio::spawn(async move {
            // let (mut rd, mut wr) = stream.split();
            let mut stream = BufStream::new(stream);
            let mut buffer = String::new();

            loop {
                match stream.read_line(&mut buffer).await {
                    Ok(0) => break, // client closed the connection;
                    Ok(_) => {
                        println!("Received: {}", buffer);
                        stream
                            .write_all(format!("Echo: {buffer}").as_bytes())
                            .await
                            .unwrap();
                        stream.flush().await.unwrap();
                    }
                    Err(e) => {
                        eprintln!("Connection errror: {e}")
                    }
                }
            }

            // if let Err(e) = handle(stream, db).await {
            //     error!("Application error: {e}");
            // }
        });
    }
}

async fn handle(stream: TcpStream, db: Db) -> crate::Result<()> {
    let mut conn = Connection::new(stream);

    match conn.read().await {
        Ok(msg) => match msg {
            None => Ok(()),
            Some(Message::Request(cmd)) => cmd.execute(db, &mut conn).await,
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
