use pm::{Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    if let Ok(d) = std::env::var("DEBUG") {
        println!("DEBUG={d}");
        env_logger::init();
    }

    let mut client = Client::connect("127.0.0.1:8421").await?;
    client.ping(None).await?;

    Ok(())
}
