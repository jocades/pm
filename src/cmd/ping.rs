use crate::{server::Response, Connection};

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

impl Ping {
    pub async fn execute(self, conn: &mut Connection) -> crate::Result<()> {
        let response = Response::ok(self.msg.unwrap_or_else(|| "Pong!".into()));

        debug!("{response:?}");
        conn.write(&response).await
    }
}
