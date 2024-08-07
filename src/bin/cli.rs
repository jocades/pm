use pm::{Client, Command, DEFAULT_PORT, LOCAL_HOST};

use clap::Parser;
use log::debug;

#[tokio::main]
async fn main() -> pm::Result<()> {
    let cli = Cli::parse();

    // cmd

    let addr = format!("{}:{}", cli.host, cli.port);
    println!("Connecting to {addr}");

    let mut client = Client::connect(&addr).await?;

    use Command::*;
    match cli.command {
        Ping(args) => debug!("Pong: {args:?}"),
        Start(args) => {
            debug!("Starting: {args:?}");
        }
        _ => unimplemented!(),
    }

    Ok(())
}

#[derive(Parser)]
#[command(version, author, propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Command,

    #[arg(long, default_value = LOCAL_HOST)]
    host: String,

    #[arg(long, default_value_t = DEFAULT_PORT)]
    port: u16,
}
