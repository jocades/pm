use crate::cmd::Executor;
use crate::db::Db;
use crate::{Command, Connection, Error, LOCAL_HOST};

use log::{error, info};
use serde_json::json;
use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};
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
            if let Err(e) = handle(stream, db).await {
                error!("Connection error: {e}");
            }

            /* while let Some(msg) = conn.read_message::<Command>().await.unwrap()
            {
                println!("Received: {msg:?}");
                // let name = msg["name"].as_str().unwrap();
                // println!("Name: {name}");

                let response = json!({
                    "status": "ok",
                    "data": msg,
                });

                conn.write_message(&response).await.unwrap();
            } */
        });
    }
}

async fn handle(stream: TcpStream, db: Db) -> crate::Result<()> {
    let mut conn = Connection::new(stream);

    match conn.read_message::<Command>().await {
        Ok(None) => Ok(()),
        Ok(Some(cmd)) => cmd.execute(db, &mut conn).await,
        Err(e) => {
            if let Error::Parse(e) = e {
                conn.write_message(&json!({"error": e})).await
            } else {
                Err(e)
            }
        }
    }
}
