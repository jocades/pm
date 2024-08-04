use clap::{Args, Parser, Subcommand};
use pm::{Action, Connection};
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::net::{Ipv4Addr, TcpStream};
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use sysinfo::{
    Components, Disks, Networks, Pid, Process, ProcessesToUpdate, System,
};

fn main() -> io::Result<()> {
    let stream = TcpStream::connect("/Users/j0rdi/.pm/pm.sock")?;

    /* let message = serde_json::to_vec(&Action::Start {
        cmd: "server.js".into(),
        name: Some("server".into()),
    })?; */

    // let mut conn = Connection::new(stream);

    let message = "Hello from client!";
    // stream.write_all(message).unwrap();
    // stream.flush().unwrap();

    println!("Sent: {:?}", message);
    Ok(())

    // let cli = Cli::parse();

    /* if let Err(e) = handle(&cli.command) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    } */
}

fn connect<P: AsRef<Path>>(path: P) -> io::Result<BufWriter<UnixStream>> {
    let stream = UnixStream::connect(path)?;
    Ok(BufWriter::new(stream))
}

// $ pm <COMMAND> [OPTIONS] [ARGS]

/// Process Manager
#[derive(Parser)]
#[command(version, about, long_about = None,)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start a new process
    Start(StartArgs),
    /// Stop a running process
    Stop {
        /// The name of the process to stop
        name: String,
    },
    /// Restart a running process
    // Restart {
    //     /// The name of the process to restart
    //     name: String,
    // },
    /// List all running processes
    Ls,
    /// Display stats for <name> process.
    Info { name: String },
}

#[derive(Args)]
struct StartArgs {
    /// The program to run
    program: PathBuf,
    /// The name of the process
    #[arg(short, long)]
    name: Option<String>,
    /// The path to the log file
    #[arg(short, long)]
    log: Option<PathBuf>,
    /// Prefix logs with a timestamp
    #[arg(short, long)]
    time: bool,
}
