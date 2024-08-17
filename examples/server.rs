use log::{error, info, warn};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::{env, fs};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufStream};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
pub async fn main() -> pm::Result<()> {
    env_logger::init();
    // let listener = TcpListener::bind(format!("{LOCAL_HOST}:{port}")).await?;
    let listener = TcpListener::bind("127.0.0.1:8421").await?;

    info!("Server started with pid: {}", std::process::id());
    info!("PM_DIR: {:?}", env::var("PM_DIR"));
    // info!("Listening on port {port}");

    loop {
        let (stream, addr) = listener.accept().await?;

        info!("Accepted {addr}");
        tokio::spawn(async move {
            let mut conn = pm::Connection::new(stream);
            while let Some(msg) = conn.read_message::<Value>().await.unwrap() {
                println!("Received: {msg}");
                // let name = msg["name"].as_str().unwrap();
                // println!("Name: {name}");

                let response = json!({
                    "status": "ok",
                    "data": msg,
                });

                conn.write_message(&response).await.unwrap();
            }
            /* // let (mut rd, mut wr) = stream.split();
            let mut stream = BufStream::new(stream);
            // let mut buffer = String::new();
            let mut buffer = Vec::with_capacity(1024);

            loop {
                buffer.clear();
                match stream.read_until(b'\n', &mut buffer).await {
                    Ok(0) => {
                        info!("{addr} closed connection");
                        break;
                    } // client closed the connection;
                    Ok(_) => {
                        let msg = String::from_utf8_lossy(&buffer);
                        println!("Received: {msg}");
                        stream
                            .write_all(format!("Echo: {msg}").as_bytes())
                            .await
                            .unwrap();
                        stream.flush().await.unwrap();
                    }
                    Err(e) => {
                        eprintln!("Connection errror: {e}")
                    }
                }
            } */

            // if let Err(e) = handle(stream, db).await {
            //     error!("Application error: {e}");
            // }
        });
    }
}
