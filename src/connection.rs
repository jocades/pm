use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufStream};
use tokio::net::TcpStream;

pub struct Connection {
    stream: BufStream<TcpStream>,
    buffer: String,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        Connection {
            stream: BufStream::new(stream),
            buffer: String::new(),
        }
    }

    pub async fn read(&mut self) -> Result<(), io::Error> {
        match self.stream.read_line(&mut self.buffer).await? {
            0 => {
                eprintln!("EOF");
            }
            _ => {
                println!("Received message, {}", self.buffer);
                self.write("OK").await?;
            }
        }

        Ok(())
    }

    pub async fn write(&mut self, msg: &str) -> Result<(), io::Error> {
        // append a '\n' to the message
        self.stream.write_all(msg.as_bytes()).await?;
        self.stream.write_all(b"\n").await?;
        self.stream.flush().await?;
        println!("Sent: {:?}", msg);
        Ok(())
    }
}
