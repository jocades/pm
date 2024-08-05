use clap::Parser;
use pm::{Client, Command, DEFAULT_PORT};

#[tokio::main]
async fn main() -> pm::Result<()> {
    let cli = Cli::parse();

    let addr = format!("{}:{}", cli.host, cli.port);
    println!("Connecting to: {}", addr);

    let mut client = Client::connect(&addr).await?;

    use pm::Command::*;
    match cli.command {
        Ping(args) => println!("Pong: {:?}", args.msg),
        Start(args) => {
            println!("Starting: {:?}", args);
        }
        _ => unimplemented!(),
    }

    Ok(())
}

#[derive(Parser, Debug)]
#[command(version, about, propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Command,

    #[arg(long, default_value_t = DEFAULT_PORT)]
    port: u16,

    #[arg(long, default_value = "127.0.0.1")]
    host: String,
}
