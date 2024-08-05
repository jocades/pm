use super::Executor;
use crate::{Connection, Response};

use clap::Args;
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
    async fn execute(&self, conn: &mut Connection) -> crate::Result<()> {
        println!("Pong");
        let res = Response::new(true, "Pong");
        conn.write(res).await
    }
}
