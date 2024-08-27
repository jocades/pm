use super::{Executor, State};
use crate::server::Response;
use crate::state::Task;
use crate::Connection;

use clap::Args;
use log::debug;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::process::Command;

// $ pm start server.js --name api
// $ pm start "bun run server.js" --name api

#[derive(Args, Serialize, Deserialize, Debug)]
pub struct Start {
    pub task: String,
    #[arg(long)]
    pub name: Option<String>,
    // #[arg(short, long)]
    // log: Option<PathBuf>,
}

impl Start {
    pub fn new<T: Into<String>>(task: T, name: Option<T>) -> Start {
        Start {
            task: task.into(),
            name: name.map(|n| n.into()),
        }
    }
}

impl Executor for Start {
    async fn execute(self, s: State, c: &mut Connection) -> crate::Result<()> {
        let name = self.name.unwrap_or_else(|| "TEST".into());

        let data = {
            let mut s = s.lock().unwrap();

            let pstdout = s.path.join(format!("{name}.log"));

            let fstdout = File::create(&pstdout)?;
            let fstderr = fstdout.try_clone()?;

            let mut command = Command::new("bun");

            command
                .args(["run", self.task.as_str()])
                .stdout(fstdout)
                .stderr(fstderr);

            debug!("Starting: {} ({})", name, self.task);

            let child = command.spawn()?;

            let pid = child.id();
            debug!("PID: {pid}");

            fs::write(s.path.join(format!("{name}.pid")), pid.to_string())?;

            let args: Vec<String> = command
                .get_args()
                .map(|s| s.to_string_lossy().to_string())
                .collect();

            let info = Task {
                name: name.clone(),
                pid,
                command: self.task,
                cpu_usage: 0.00,
                mem_usage: 0,
            };

            s.db.insert(name, info.clone());

            info
        };

        debug!("{s:?}");

        c.write(&Response::ok(data)).await?;
        Ok(())
    }
}
