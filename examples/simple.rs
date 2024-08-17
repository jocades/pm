use log::info;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{thread, time::Duration};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::io::{BufReader, BufWriter};

// it could be any message that we want to send
#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    age: u8,
}

#[derive(Debug, Serialize, Deserialize)]
enum Command {
    Start {
        name: String,
        command: String,
        args: Vec<String>,
    },
    Stop {
        name: String,
    },
    List,
}

#[derive(Debug, Serialize, Deserialize)]
enum Response {
    Ok(Value),
    Error(String),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // create an in memory duplex stream that simulates a TCP connection.
    let (mock_reader, mut mock_writer) = tokio::io::duplex(1024);

    tokio::spawn(async move {
        let cmd = Command::Start {
            name: "list".to_string(),
            command: "ls".to_string(),
            args: vec!["-la".to_string()],
        };

        // serialize the command to JSON
        let data = serde_json::to_vec(&cmd).unwrap();

        // write the message length as u32 pefix (4 bytes);
        let len = data.len() as u32;
        mock_writer.write_u32(len).await.unwrap();

        // write the json data
        mock_writer.write_all(&data).await.unwrap();

        // flush the writer to ensure all data is sent
        mock_writer.flush().await.unwrap();
    });

    // read the data using a buffer
    let mut reader = BufReader::new(mock_reader);

    // read the message length
    let len = reader.read_u32().await.unwrap() as usize;

    // the the exact number of bytes for the rest of the message
    let mut buffer = vec![0u8; len];
    reader.read_exact(&mut buffer).await.unwrap();

    // deserialize the json
    let received: Command = serde_json::from_slice(&buffer).unwrap();

    println!("Received: {:?}", received);

    Ok(())
}
