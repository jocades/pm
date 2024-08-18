mod ping;
pub use ping::Ping;

mod start;
pub use start::Start;

mod stop;
pub use stop::Stop;

use crate::Connection;
use crate::State as BaseState;

use clap::Subcommand;
use derive_more::From;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Subcommand, Serialize, Deserialize, From, Debug)]
pub enum Command {
    Ping(Ping),
    Start(Start),
    Stop(Stop),
}

pub(crate) type State = Arc<Mutex<BaseState>>;

pub(crate) trait Executor {
    async fn execute(self, s: State, c: &mut Connection) -> crate::Result<()>;
}

impl Executor for Command {
    async fn execute(self, s: State, c: &mut Connection) -> crate::Result<()> {
        use Command::*;
        match self {
            Ping(cmd) => cmd.execute(c).await,
            Start(cmd) => cmd.execute(s, c).await,
            Stop(cmd) => cmd.execute(s, c).await,
        }
    }
}
