#![allow(unused_imports)]

use super::{Executor, State};
use crate::{server::Response, Connection};

use sysinfo::{Pid, Process, ProcessesToUpdate, System};

use clap::Args;
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Args, Serialize, Deserialize, Debug)]
pub struct Stop {
    pub name: String,
}

impl Stop {
    pub fn new<T: Into<String>>(name: T) -> Stop {
        Stop { name: name.into() }
    }
}

impl Executor for Stop {
    async fn execute(self, s: State, c: &mut Connection) -> crate::Result<()> {
        info!("Stopping: {}", self.name);

        let response = {
            let s = s.lock().unwrap();
            match s.db.get(&self.name) {
                None => Response::err(format!("{} not found", &self.name)),
                Some(info) => {
                    let p =
                        s.sys.process(Pid::from(info.pid as usize)).unwrap();

                    if !p.kill() {
                        Response::err(format!(
                            "failed to kill process `{}`",
                            self.name
                        ))
                    } else {
                        Response::ok(format!(
                            "killed `{}` ({})",
                            self.name,
                            p.pid()
                        ))
                    }
                }
            }
        };

        c.write(&response).await?;
        Ok(())
    }
}
