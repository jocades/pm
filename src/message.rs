use crate::Command;

use derive_more::From;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, From)]
pub enum Message {
    Request(Command),
    Response(Response),
}

#[derive(Serialize, Deserialize, Debug, From)]
pub struct Response {
    pub ok: bool,
    pub msg: String,
}

impl Response {
    pub fn new<T: Into<String>>(ok: bool, msg: T) -> Response {
        Response {
            ok,
            msg: msg.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cmd::Ping;

    #[test]
    fn request() {
        let cmd: Command = Ping::new(Some("hello")).into();
        let msg: Message = cmd.into();

        assert!(matches!(msg, Message::Request(_)));
    }

    #[test]
    fn response() {
        let res = Response::new(true, "pong");
        let msg: Message = res.into();

        assert!(matches!(msg, Message::Response(_)));
    }
}
