use super::Executor;
use crate::Connection;

use clap::Args;
use serde::{Deserialize, Serialize};

#[derive(Args, Serialize, Deserialize, Debug)]
pub struct Stop {
    name: String,
}

impl Executor for Stop {
    async fn execute(&self, _conn: &mut Connection) -> crate::Result<()> {
        println!("Stopping: {}", self.name);
        Ok(())
    }
}
