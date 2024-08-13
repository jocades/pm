use pm::{Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = Client::connect("127.0.0.1:8421").await?;

    client.ping().await?;

    Ok(())
}
