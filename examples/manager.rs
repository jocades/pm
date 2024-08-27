#![allow(unused)]

use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use sysinfo::{Pid, Process, ProcessesToUpdate, System};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let task = "bun run server.js";

    let mut args = task.split_whitespace();
    let cmd = args.next().unwrap();
    let args: Vec<&str> = args.collect();

    let mut manager = Manager::new(env::current_dir()?);
    let mut command = Command::new(cmd);

    command
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    let child = command.spawn()?;

    manager.db.insert(cmd.into(), child.id());
    let child = manager.db.get(cmd).unwrap();

    println!("{:?}", manager.sys.physical_core_count());

    update_loop(&mut manager);

    Ok(())
}

fn update_loop(manager: &mut Manager) {
    loop {
        manager.update();

        for (name, task) in manager.tasks() {
            println!(
                "{} ({}), cpu: {}%, mem: {}MB",
                // task.name().to_str().unwrap(),
                name,
                task.pid(),
                // task.cpu_usage(),
                task.cpu_usage() / 10.0,
                task.memory() / 1024 / 1024
            );
        }

        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    }
}

#[derive(Debug)]
struct Manager {
    db: HashMap<String, u32>,
    sys: System,
    path: PathBuf,
}

impl Manager {
    fn new<P: Into<PathBuf>>(path: P) -> Manager {
        Manager {
            db: HashMap::new(),
            sys: System::new(),
            path: path.into(),
        }
    }

    pub fn get(&self, pid: u32) -> Option<&Process> {
        self.sys.process(Pid::from_u32(pid))
    }

    pub fn get_by_name(&self, name: &str) -> Option<&Process> {
        let pid = self.db.get(name)?;
        self.get(*pid)
    }

    pub fn update(&mut self) {
        self.sys.refresh_processes(ProcessesToUpdate::Some(
            self.db
                .values()
                .map(|&pid| Pid::from_u32(pid))
                .collect::<Vec<Pid>>()
                .as_slice(),
        ));
    }

    pub fn tasks(&self) -> Vec<(String, &Process)> {
        self.db
            .iter()
            .map(|(name, &pid)| (name.clone(), self.get(pid).unwrap()))
            .collect()
    }
}
