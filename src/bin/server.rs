use pm::{server, DEFAULT_PORT};

use clap::Parser;
use std::env;
use std::path::Path;

#[tokio::main]
async fn main() -> pm::Result<()> {
    env_logger::init();

    let wd = Path::new(env!("HOME")).join(".pm");
    env::set_var("PM_DIR", &wd);

    let cli = Cli::parse();

    server::run(cli.port).await
}

#[derive(Parser)]
#[command(name = "pm-server", version)]
struct Cli {
    #[arg(long, default_value_t = DEFAULT_PORT)]
    port: u16,
}
