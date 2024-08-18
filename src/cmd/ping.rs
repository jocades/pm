use crate::Connection;

use clap::Args;
use log::debug;
use serde::{Deserialize, Serialize};
use serde_json::json;

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
        let response = json!({
            "status": "ok",
            "data": self.msg.unwrap_or_else(|| "Pong!".into()),
        });
        debug!("{response:?}");
        conn.write_message(&response).await
    }
}
