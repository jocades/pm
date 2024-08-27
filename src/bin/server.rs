use pm::{server, DEFAULT_PORT};

use clap::Parser;

#[tokio::main]
async fn main() -> pm::Result<()> {
    env_logger::init();

    let cli = Cli::parse();

    server::run(cli.port).await
}

#[derive(Parser)]
#[command(name = "pm-server", version)]
struct Cli {
    #[arg(long, default_value_t = DEFAULT_PORT)]
    port: u16,
}
