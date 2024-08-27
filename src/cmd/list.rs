use super::{Executor, State};
use crate::{server::Response, Connection};

use clap::Args;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Args, Serialize, Deserialize, Debug)]
pub struct List;

impl Executor for List {
    async fn execute(self, s: State, c: &mut Connection) -> crate::Result<()> {
        let response = {
            let s = s.lock().unwrap();

            let tasks: Vec<_> = s.db.values().collect();
            Response::ok(json!(&tasks))
        };

        c.write(&response).await?;
        Ok(())
    }
}
