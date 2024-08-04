use pm::Connection;
use serde::{Deserialize, Serialize};
use std::io;
use tokio::net::TcpStream;

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    content: String,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:6379").await?;

    // let mut writer = BufWriter::new(stream.try_clone()?);

    // let msg = b"Hello from client!\n";
    // println!("Sending: {}", String::from_utf8_lossy(msg));
    // println!("Sending: {:?}", msg);

    // create a json message with a '\n' at then end

    /* msg.push(b'\n');

    println!("Sending: {:?}", msg);

    stream.write_all(&msg)?; */

    // write directly to the writer
    /* serde_json::to_writer(
        &stream,
        &Message {
            content: "Hello from client!".to_string(),
        },
    )?;
    writer.write_all(b"\n")?;
    writer.flush()?;

    // read the response
    let mut reader = BufReader::new(&stream);
    let mut buffer = String::new();

    reader.read_line(&mut buffer)?;
    println!("Server responded with: {}", buffer);

    writer.write_all(b"OK\n")?;
    writer.flush()?; */

    // listen to the response
    // let mut reader = BufWriter::new(stream);
    // let mut buffer = String::new();
    // reader.write_all(msg.as_bytes())?;
    // reader.flush()?;

    // writer.write_all(msg.as_bytes())?;
    // writer.flush()?;

    let mut conn = Connection::new(stream);

    let msg = "Hello from client!";
    conn.write(msg).await?;
    conn.read().await?;
    // conn.write("OK").await?;

    Ok(())

    // let cli = Cli::parse();

    /* if let Err(e) = handle(&cli.command) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    } */
}
