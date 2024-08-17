use super::Executor;
use crate::db::Db;
use crate::{Connection, Response};

use clap::Args;
use log::debug;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::process::{Command, Stdio};
use std::{env, fs};
use sysinfo::Pid;

/// $ pm start server.js --name api
/// $ pm start "bun run server.js" --name api

#[derive(Args, Serialize, Deserialize, Debug)]
pub struct Start {
    pub process: String,
    #[arg(long)]
    pub name: Option<String>,
    // #[arg(short, long)]
    // log: Option<PathBuf>,
}

impl Start {
    pub fn new<T: Into<String>>(process: T, name: Option<T>) -> Start {
        Start {
            process: process.into(),
            name: name.map(|n| n.into()),
        }
    }
}

use std::path::PathBuf;
use std::process;

pub fn get_dir() -> PathBuf {
    match env::var("PM_DIR") {
        Ok(dir) => PathBuf::from(dir),
        Err(_) => {
            eprintln!("Error: $PM_DIR not found");
            process::exit(1);
        }
    }
}

impl Executor for Start {
    async fn execute(self, db: Db, conn: &mut Connection) -> crate::Result<()> {
        let name = self.name.unwrap_or_else(|| "TEST".into());

        let path = get_dir().join(&name);
        fs::create_dir_all(&path)?;

        let stdout = File::create(path.join("pm.out"))?;
        let stderr = File::create(path.join("pm.err"))?;

        let mut command = Command::new("bun");

        command
            .args(["run", self.process.as_str()])
            .stdout(Stdio::from(stdout))
            .stderr(Stdio::from(stderr));

        debug!("Starting: {} ({})", name, self.process);

        let child = command.spawn()?;

        let pid = Pid::from(child.id() as usize);
        println!("PID: {pid}");

        writeln!(File::create(path.join("pid"))?, "{pid}")?;

        {
            let mut db = db.lock().unwrap();
            db.insert(name, pid);
        }

        debug!("{db:?}");

        conn.write(Response::Ok(pid.to_string())).await
    }
}
