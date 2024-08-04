#![allow(unused_imports)]

use ctrlc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::{env, fs};
use sysinfo::{
    Components, Disks, Networks, Pid, Process, ProcessesToUpdate, System,
};

/* enum Req {
    Start { name: String, cmd: String },
    Stop { name: String },
} */

fn main() -> io::Result<()> {
    let dir = env::temp_dir().join("bun");
    let endpoint = dir.join("pm.sock");
    let listener = UnixListener::bind(&endpoint)?;

    println!("Listening on {:?}", endpoint);

    let mut manager = ProcessManager::new(dir);

    loop {
        let (stream, _) = listener.accept()?;
        let mut reader = BufReader::new(&stream);
        let mut buffer = String::new();

        match reader.read_line(&mut buffer) {
            Ok(0) => {
                eprintln!("EOF");
            }
            Ok(_) => match serde_json::from_str::<Action>(&buffer) {
                Ok(action) => {
                    println!("received: {:?}", action);
                    match action {
                        Action::Start { cmd, name } => {
                            manager.start(cmd, name)?
                        }
                        Action::Stop { name } => manager.stop(&name)?,
                        _ => unimplemented!(),
                    }
                }
                Err(e) => {
                    eprintln!("invalid request format: {}", e);
                }
            },
            Err(e) => {
                eprintln!("error reading from socket: {}", e);
            }
        }

        buffer.clear();

        // reader.read_line(&mut buffer)?;
        /* let line = &buffer.trim();

        match serde_json::from_str::<Action>(line) {
            Err(_) => {
                eprintln!("Invalid request format {line}")
            }
            Ok(action) => {
                println!("Received: {:?}", action);
                match action {
                    Action::Start { cmd, name } => manager.start(cmd, name)?,
                    _ => panic!("Not implemented"),
                }
            }
        } */

        /* match serde_json::from_str(&buffer.trim()) {
            Ok(Start { name, cmd }) => {
                println!("Starting: {} ({})", name, cmd);
            }
            Err(e) => {
                eprintln!("Invalid request: {}", e);
            }
        } */
    }
}

struct ProcessManager {
    pub dir: PathBuf,
    pub sys: System,
    pub processes: HashMap<String, Pid>,
}

impl ProcessManager {
    pub fn new(dir: PathBuf) -> Self {
        ProcessManager {
            dir,
            sys: System::new(),
            processes: HashMap::new(),
        }
    }

    pub fn get(&mut self, name: &str) -> Option<&Process> {
        let pid = self.processes.get(name)?;
        self.sys.process(*pid)
    }

    pub fn set(&mut self, name: impl Into<String>, id: u32) -> io::Result<()> {
        self.processes.insert(name.into(), Pid::from(id as usize));
        fs::write(&self.dir, id.to_string())?;
        Ok(())
    }

    pub fn start(
        &mut self,
        cmd: String,
        _name: Option<String>,
    ) -> io::Result<()> {
        let mut command = Command::new("bun");

        command
            .args(["run", &cmd])
            .stdout(Stdio::null())
            .stderr(Stdio::null());

        let child = command.spawn()?;
        self.set("test", child.id())?;

        Ok(())
    }

    pub fn stop(&mut self, name: &str) -> io::Result<()> {
        let pid = self.processes.get(name).unwrap();
        Command::new("kill").arg(pid.to_string()).spawn()?.wait()?;

        Ok(())
    }
}
