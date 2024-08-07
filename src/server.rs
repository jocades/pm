use crate::{Connection, LOCAL_HOST};

use log::info;
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
        tokio::spawn(async move { process(stream, db).await });
    }
}

async fn process(stream: TcpStream, _db: Db) -> crate::Result<()> {
    let mut conn = Connection::new(stream);
    let _msg = conn.read().await?;
    // println!("Received: {:?}", msg);
    Ok(())
}
