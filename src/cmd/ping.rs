use super::Executor;
use crate::{db::Db, Connection, Response};

use clap::Args;
use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Args, Serialize, Deserialize, Debug)]
pub struct Ping {
    pub msg: Option<String>,
}

impl Ping {
    pub fn new<T: Into<String>>(msg: Option<T>) -> Ping {
        Ping {
            msg: msg.map(|s| s.into()),
        }
    }
}

impl Executor for Ping {
    async fn execute(self, db: Db, conn: &mut Connection) -> crate::Result<()> {
        let res = Response::Ok(self.msg.unwrap_or_else(|| "Pong!".into()));
        debug!("{res:?}");
        conn.write(res).await
    }
}
