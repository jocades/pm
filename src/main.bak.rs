// #![allow(unused)] // for dev

// mod daeomon;

use clap::{Args, Parser, Subcommand};
use std::error::Error;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::{env, process};
use sysinfo::{
    Components, Disks, Networks, Pid, Process, ProcessesToUpdate, System,
};

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

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() {
    let cli = Cli::parse();

    if let Err(e) = handle(&cli.command) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn handle(cmd: &Commands) -> Result<()> {
    let mut sys = System::new();

    use Commands::*;
    match cmd {
        Start(args) => {
            start(&args.program, args.name.as_deref(), args.log.as_deref())
        }
        Stop { name } => stop(name),
        Ls => list(),
        Info { name } => info(&mut sys, name),
    }
}

fn get_pid(name: &str) -> Result<Pid> {
    let pid_path = pm::get_dir().join(name).join("pid");
    let pid = fs::read_to_string(&pid_path)?.trim().parse::<usize>()?;

    Ok(Pid::from(pid))
}

fn refresh(sys: &mut System, pids: &[Pid]) {
    sys.refresh_processes(ProcessesToUpdate::Some(pids));
}

fn info(sys: &mut System, name: &str) -> Result<()> {
    let pid = get_pid(name)?;
    refresh(sys, &[pid]);
    let p = sys
        .process(pid)
        .ok_or(format!("Process `{name} ({pid}) not found"))?;

    println!(
        "{name} ({pid}), cpu: {}, mem: {}",
        p.cpu_usage(),
        p.memory()
    );

    Ok(())
}

fn start(path: &PathBuf, name: Option<&str>, log: Option<&Path>) -> Result<()> {
    let name = name.unwrap_or_else(|| {
        path.file_stem()
            .expect("Unable to get file stem")
            .to_str()
            .expect("Unable to convert OsStr to str")
    });

    let ps_dir = pm::get_dir().join(name);
    fs::create_dir_all(&ps_dir)?;

    let pid_path = ps_dir.join("pid");

    let stdout = File::create(&ps_dir.join("out"))?;
    let stderr = File::create(&ps_dir.join("err"))?;

    let mut command = Command::new("bun");

    command
        .args(["run", path.to_str().unwrap()])
        .stdout(Stdio::from(stdout))
        .stderr(Stdio::from(stderr));

    let child = command.spawn().expect("Failed to start process");
    writeln!(File::create(&pid_path)?, "{}", child.id())?;

    /* if let Some(log) = log {
        let log_file = File::create(log)?;
        command.stdout(Stdio::from(log_file.try_clone()?));
        command.stderr(Stdio::from(log_file));
    } */

    println!("Started process: {}", name);

    Ok(())
}

fn list() -> Result<()> {
    let pm_dir = pm::get_dir();
    let entries = fs::read_dir(&pm_dir)?;

    for entry in entries {
        let entry = entry?;
        let name = entry.file_name();
        let name = name.to_str().expect("Unable to convert OsStr to str");

        if name.ends_with(".pid") {
            let name = name.trim_end_matches(".pid");
            let pid = fs::read_to_string(entry.path())?;
            println!("{name} ({pid})");
        }
    }

    Ok(())
}

fn get(name: &str) -> Option<(u32, PathBuf, PathBuf)> {
    let pm_dir = pm::get_dir();
    let pid_path = pm_dir.join(format!("{name}.pid"));

    let pid = fs::read_to_string(&pid_path)
        .ok()?
        .trim()
        .parse::<u32>()
        .ok()?;

    let stdout_path = pm_dir.join(format!("{name}.out"));
    let stderr_path = pm_dir.join(format!("{name}.err"));

    Some((pid, stdout_path, stderr_path))
}

fn stop(name: &str) -> Result<()> {
    let pid_path = pm::get_dir().join(name).join("pid");
    let pid = fs::read_to_string(&pid_path)?;
    let pid = pid.trim();
    // let pid = pid.trim().parse::<u32>().expect("Invalid pid");

    let mut kill = Command::new("kill")
        .arg(pid)
        .status()
        .expect("Failed to kill process");

    if (!kill.success()) {
        panic!("Failed to kill process `{name}`")
    }

    fs::remove_file(pid_path)?;

    println!("Stopped process: {name} ({pid})");

    Ok(())
}

/* fn restart(name: &str) -> Result<(), Box<dyn Error>> {
    stop(name)?;
    let (pid, stdout, stderr) = get(name).expect("Unable to get process info");
    let program = env::current_exe().expect("Unable to get current executable");

    start(&program, Some(name), Some(stdout))?;

    Ok(())
} */
