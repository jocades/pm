use pm::{server, DEFAULT_PORT};

use clap::Parser;
use log::info;
use std::env;
use std::path::Path;
use std::process;

#[tokio::main]
async fn main() -> pm::Result<()> {
    env_logger::init();

    let path = Path::new(env!("HOME")).join(".pm");
    env::set_current_dir(&path)?;

    // log current pid and other info
    info!("Server started with pid: {}", process::id());
    info!("Working directory: {path:?}");

    /* let stdout = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("pm.out")?;

    let stderr = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("pm.err")?; */

    let cli = Cli::parse();

    server::run(cli.port).await
}

#[derive(Parser)]
#[command(name = "pm-server", version)]
struct Cli {
    #[arg(long, default_value_t = DEFAULT_PORT)]
    port: u16,
}
