use pm::Connection;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader, BufStream};
use tokio::net::{TcpListener, TcpStream};

type Db = Arc<Mutex<HashMap<String, u32>>>;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    println!("Listening");

    let db: Db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        // Clone the handle to the hash map. (Increment the reference count)
        let db = db.clone();

        println!("Accepted");
        tokio::spawn(async move {
            process(stream, db).await;
        });
    }
}

async fn process(stream: TcpStream, db: Db) {
    /* // let mut reader = BufReader::new(stream);
    let mut stream = BufStream::new(stream);
    let mut buffer = String::new();

    loop {
        // let line = reader.read_until(b'x', &mut buffer).await.unwrap();
        match stream.read_line(&mut buffer).await.unwrap() {
            0 => {
                println!("EOF detectd");
                // break;
                return;
            }
            _ => {
                println!("Received message, {}", buffer);
                let response = "OK\n";
                stream.write_all(response.as_bytes()).await.unwrap();
                stream.flush().await.unwrap();
                buffer.clear();
            }
        }
    } */

    let mut conn = Connection::new(stream);
    conn.read().await.unwrap();

    /* use mini_redis::Command::{self, Get, Set};

    // Connection, provided by `mini-redis`, handles parsing frames from
    // the socket
    let mut connection = Connection::new(socket);

    // Use `read_frame` to receive a command from the connection.
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        // Write the response to the client
        connection.write_frame(&response).await.unwrap();
    } */
}
