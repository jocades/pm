use super::Executor;
use crate::{db::Db, Connection};

use clap::Args;
use serde::{Deserialize, Serialize};

#[derive(Args, Serialize, Deserialize, Debug)]
pub struct Stop {
    name: String,
}

impl Executor for Stop {
    async fn execute(self, db: Db, conn: &mut Connection) -> crate::Result<()> {
        println!("Stopping: {}", self.name);
        Ok(())
    }
}
