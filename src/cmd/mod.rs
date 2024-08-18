mod ping;
pub use ping::Ping;

mod start;
pub use start::Start;

mod stop;
use stop::Stop;

use crate::{db::Db, Connection};

use clap::Subcommand;
use derive_more::From;
use serde::{Deserialize, Serialize};

#[derive(Subcommand, Serialize, Deserialize, From, Debug)]
pub enum Command {
    Ping(Ping),
    Start(Start),
    Stop(Stop),
}

pub(crate) trait Executor {
    async fn execute(self, db: Db, conn: &mut Connection) -> crate::Result<()>;
}

impl Executor for Command {
    async fn execute(self, db: Db, conn: &mut Connection) -> crate::Result<()> {
        use Command::*;
        match self {
            Ping(cmd) => cmd.execute(conn).await,
            Start(cmd) => cmd.execute(db, conn).await,
            Stop(cmd) => cmd.execute(db, conn).await,
        }
    }
}
