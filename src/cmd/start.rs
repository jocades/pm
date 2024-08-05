use super::{Command, Executor};
use crate::{Connection, Message};

use clap::Args;
use serde::{Deserialize, Serialize};

#[derive(Args, Serialize, Deserialize, Debug)]
pub struct Start {
    process: String,
    name: String,
}

impl Start {
    pub fn new<T: Into<String>>(process: T, name: Option<T>) -> Start {
        let default = "test";
        Start {
            process: process.into(),
            name: name.map_or(default.into(), |n| n.into()),
        }
    }
}

impl Executor for Start {
    async fn execute(&self, _conn: &mut Connection) -> crate::Result<()> {
        println!("Starting: {} ({})", self.name, self.process);
        Ok(())
    }
}
